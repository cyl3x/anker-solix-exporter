mod api;
mod credentials;
pub mod data;

pub use api::{Error, SolixApi};
pub use credentials::Credentials;

// pub struct Solix {
//     api: api::SolixApi,
//     user: Option<api::User>,
//     username: String,
//     password: String,
// }

// impl Solix {
//     pub fn new(username: String, password: String, country: String, timezone: String) -> Self {
//         Self {
//             api: api::SolixApi::new(country, timezone),
//             user: None,
//             username,
//             password,
//         }
//     }

//     pub fn set_user(&mut self, user: api::User) {
//         self.user = Some(user);
//     }

//     pub fn get_user(&self) -> Option<&api::User> {
//         self.user.as_ref()
//     }

//     pub fn ensure_login(&mut self) -> Result<(), api::Error> {
//         if let Some(user) = self.user.as_ref() {
//             if user.is_expired() {
//                 self.login()?;
//             }
//         } else {
//             self.login()?;
//         }

//         Ok(())
//     }

//     pub fn login(&mut self) -> Result<(), api::Error> {
//         match self.api.login(&self.username, &self.password) {
//             Ok(user) => {
//                 self.user = Some(user.into());
//                 Ok(())
//             }
//             Err(err) => Err(err),
//         }
//     }

//     pub fn scen_info(&mut self, site_id: &str) -> Result<data::ScenInfo, api::Error> {
//         self.ensure_login()?;

//         match self.api.get_scen_info(, site_id) {
//             Ok(data) => Ok(data),
//             Err(api::Error::InvalidCredentials) => {
//                 self.user = None;
//                 self.login()?;

//                 self.api.get_scen_info(self.get_user().unwrap(), site_id)
//             }
//             Err(err) => Err(err),
//         }
//     }
// }
