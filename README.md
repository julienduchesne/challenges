# Challenges

An embarrassingly complex showcase of challenge solutions. This serves as a playground to try out new languages. A new group of challenges can be added by adding a new independent module (microservice), it can therefore be implemented in any language

## How to run it

To launch the complete solution, use `make run-all -j4`

Environment Variables:

* `CHALLENGES_APP_PORT` (default = 8080)
* `CHALLENGES_API_PORT` (default = 8081)
* `CHALLENGES_AOC_2019_PORT` (default = 8082)
* `CHALLENGES_AOC_2018_PORT` (default = 8083)

## Services

### Backend

Language: Rust

Exposes through an API ([Rocket](https://rocket.rs/)) and a Text User Interface ([Cursive](https://github.com/gyscos/cursive)) the solutions for the Advent of Code 2020. It also fronts the solutions for other challenge series (modules listed below)

### Advent of Code 2019 Module

Language: Go

Exposes, through a simple HTTP API, the solutions for Advent of Code 2019

### Advent of Code 2018 Module

Language: Bash

Exposes, through a simple (and frankly bad) HTTP API, the solutions for Advent of Code 2018

### Frontend

Language: Typescript

React frontend
