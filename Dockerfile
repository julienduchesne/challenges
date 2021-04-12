FROM rustlang/rust:nightly-buster AS rust-builder
COPY ["Cargo.toml", "Cargo.lock", "src", "./"]
RUN cargo build --release

FROM golang:buster AS go-builder
COPY ["src/groups/advent_of_code_2019", "./"]
RUN GOPATH= GO111MODULE=on CGO_ENABLED=0 go build -o /advent_of_code_2019

FROM debian:buster

COPY --from=rust-builder target/release/api /opt/challenges/api
COPY --from=go-builder /advent_of_code_2019 /opt/challenges/advent_of_code_2019


ENTRYPOINT ["/opt/challenges/api"]