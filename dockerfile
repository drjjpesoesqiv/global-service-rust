FROM rust:latest
WORKDIR /usr/src/global-service
COPY . .
RUN cargo build --release
RUN cargo install --path .
EXPOSE 7777:7777
CMD ["/usr/local/cargo/bin/global-service"]
