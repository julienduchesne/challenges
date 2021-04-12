.PHONY: test run-frontend run-backend build-image run-image

IMAGE_NAME := "julienduchesne/challenges"

test:
	cd src/groups/advent_of_code_2019 && go test ./...
	cargo test

run-frontend:
	
run-backend:

build-image:
	docker build . -t $(IMAGE_NAME)

run-image: build-image
	docker run $(IMAGE_NAME)