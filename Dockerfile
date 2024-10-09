FROM rust:1.81.0-bookworm as build

# create a new empty shell project
RUN USER=root cargo new --bin api-naroden-org
WORKDIR /api-naroden-org

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/api_naroden_org*
RUN cargo build --release

# our final base
FROM debian:bookworm-slim

# copy the build artifact from the build stage
COPY --from=build /api-naroden-org/target/release/api-naroden-org .

# set the startup command to run your binary
CMD ["./api-naroden-org"]
