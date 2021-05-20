#!/usr/bin/env bats

helper() {
    result="$(echo "$1" | ./day01.sh)"
    echo "Result: $result"
    [ "$result" == "$(echo -e "$2")" ]
}

@test "Day 01: 1" {
    helper "+1, -1" "Part 1: 0\nPart 2: 0"
}

@test "Day 01: 2" {
    helper "+3, +3, +4, -2, -4" "Part 1: 4\nPart 2: 10"
}

@test "Day 01: 3" {
    helper "-6, +3, +8, +5, -6" "Part 1: 4\nPart 2: 5"
}

@test "Day 01: 4" {
    helper "+7, +7, -2, -7, -4" "Part 1: 1\nPart 2: 14"
}

@test "Day 01: Real" {
    helper "$(cat ./day01_input.txt)" "Part 1: 493\nPart 2: 413"
}
