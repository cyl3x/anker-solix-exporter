mod config;
mod metrics;
mod solix;
use std::net::SocketAddr;
use std::process;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub use config::Config;
pub use metrics::Metrics;
use signal_hook::consts::SIGINT;
use signal_hook::consts::SIGTERM;
use signal_hook::flag;
use solix::Credentials;
use solix::SolixApi;
use tiny_http::{Response, Server};

struct App {
    config: Config,
    credentials: Option<Credentials>,
    metrics: Metrics,
    solix: SolixApi,
}

impl App {
    fn login(&mut self, force: bool) -> bool {
        if let Some(creds) = &self.credentials {
            let expires_in = creds.expires_in().unwrap();
            if expires_in > 0 && !force {
                log::info!("Credentials are still valid for {expires_in} seconds");
                return true;
            }
        }

        match self
            .solix
            .login(self.config.username(), self.config.password())
        {
            Ok(login) => {
                log::info!("Logged in successfully");
                let creds: Credentials = login.into();
                self.credentials = Some(creds.save(self.config.cache_file()));

                true
            }
            Err(solix::Error::InvalidCredentials) => {
                log::error!("Invalid credentials");
                false
            }
            Err(err) => {
                log::error!("Failed to login: {err}");
                true
            }
        }
    }

    fn update_metrics(&mut self, retried: bool) -> bool {
        self.login(false);

        let Some(creds) = &self.credentials else {
            return false;
        };

        match self.solix.get_scen_info(creds, self.config.scene_id()) {
            Ok(data) => {
                log::info!("Metrics updated successfully");
                self.metrics.update(data);
                true
            }
            Err(solix::Error::InvalidCredentials) => match retried {
                true => false,
                false => {
                    self.login(true);
                    self.update_metrics(true)
                }
            },
            Err(solix::Error::Api(10000, _)) => {
                log::error!("Failed to retrieve scen info: Invalid request, check COUNTRY, TIMEZONE, and SCENE_ID");
                false
            }
            Err(err) => {
                log::error!("Failed to get scen info: {err}");
                false
            }
        }
    }

    pub fn address(&self) -> SocketAddr {
        self.config.address()
    }

    pub fn get_metrics(&mut self) -> Option<String> {
        match self.update_metrics(false) {
            true => Some(self.metrics.gather()),
            false => None,
        }
    }
}

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = match Config::new() {
        Ok(metadata) => metadata,
        Err(err) => {
            log::error!("{err}");
            process::exit(1);
        }
    };

    let mut app = App {
        metrics: Metrics::new(),
        solix: SolixApi::new(config.country(), config.timezone()),
        credentials: Credentials::load(config.cache_file()),
        config,
    };

    app.get_metrics();

    let server = Server::http(app.address()).unwrap();

    let _ = flag::register_conditional_shutdown(SIGINT, 0, Arc::new(AtomicBool::new(true)));
    let _ = flag::register_conditional_shutdown(SIGTERM, 0, Arc::new(AtomicBool::new(true)));

    for request in server.incoming_requests() {
        let result = match app.get_metrics() {
            Some(metrics) => request.respond(Response::from_string(metrics)),
            None => request.respond(Response::empty(500)),
        };

        if let Err(err) = result {
            log::error!("Failed to responde: {}", err);
        }
    }
}
