# Build commands (Release builds)
.PHONY: build-backend build-frontend build-aoc2019 build

build-backend:
	cd backend && cargo build --release

build-frontend:
	cd frontend && npm run build

build-aoc2019:
	cd advent_of_code_2019 && CGO_ENABLED=0 go build ./...

build: build-backend build-frontend build-aoc2019


# Run commands (Debug/fast builds)
.PHONY: run-backend run-frontend run-aoc2019 run-all

run-backend:
	cd backend && cargo run --bin api

run-frontend:
	cd frontend && npm start

run-aoc2019:
	cd advent_of_code_2019 && go run ./...

run-aoc2018:
	cd advent_of_code_2018 && bash server.sh

# Run with -j4
run-all: run-backend run-frontend run-aoc2019 run-aoc2018

# Test commands

.PHONY: test-backend test-aoc2019 test-aoc2018

test-backend:
	cd backend && cargo test

test-aoc2019:
	cd advent_of_code_2019 && go test ./...

test-aoc2018:
	cd advent_of_code_2018 && shellcheck -axP SCRIPTDIR *.sh
	cd advent_of_code_2018 && bats . --jobs 4

test: test-backend test-aoc2019 test-aoc2018