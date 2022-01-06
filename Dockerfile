FROM rust:slim
COPY ./target/release/position-manager ./target/release/position-manager
ENTRYPOINT ["./target/release/position-manager"]