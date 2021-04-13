.PHONY: build test

IMAGE_NAME := "julienduchesne/challenges"

build:
	cd advent_of_code_2019 && CGO_ENABLED=0 go build ./...
	cd backend && cargo build --release

test:
	cd advent_of_code_2019 && go test ./...
	cd backend && cargo test