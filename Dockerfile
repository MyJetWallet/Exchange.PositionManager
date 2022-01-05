FROM rust:slim
COPY ./target/release/position-manager ./target/release/position-manager
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/position-manager"]