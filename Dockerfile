FROM rust:bullseye as build
COPY . /app
WORKDIR /app
RUN cargo build --release

FROM debian:11
RUN apt update && apt full-upgrade -y
RUN apt install -y curl openssl libssl-dev
COPY --from=build /app/target/release/saku /app/saku
WORKDIR /app
RUN chmod +x saku
ENTRYPOINT [ "./saku" ]