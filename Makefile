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

# Run with -j3
run-all: run-backend run-frontend run-aoc2019


.PHONY: test
test:
	cd advent_of_code_2019 && go test ./...
	cd backend && cargo test