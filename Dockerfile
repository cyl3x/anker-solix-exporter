FROM nixos/nix as builder

COPY . /tmp/build
WORKDIR /tmp/build

RUN nix \
    --extra-experimental-features 'nix-command flakes' \
    build

RUN mkdir /tmp/nix-store-closure
RUN cp -R $(nix-store -qR result/) /tmp/nix-store-closure

FROM scratch

COPY --from=builder /tmp/nix-store-closure /nix/store
COPY --from=builder /tmp/build/result/bin/anker-solix-exporter /anker-solix-exporter

ENV ANKER_SOLIX_ADDRESS 0.0.0.0:8080
ENV ANKER_SOLIX_CACHE_FILE /app/token_cache.json
ENV ANKER_SOLIX_COUNTRY DE
ENV RUST_LOG info

ENTRYPOINT ["/anker-solix-exporter"]
