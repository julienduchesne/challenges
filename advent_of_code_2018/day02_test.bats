#!/usr/bin/env bats

helper() {
    result="$(echo "$1" | ./day02.sh)"
    echo "Result: $result"
    [ "$result" == "$(echo -e "$2")" ]
}

@test "Day 02: Simple" {
    helper "abcdef bababc abbcde abcccd aabcdd abcdee ababab" "Part 1: 12\nPart 2: abcde"
}

@test "Day 02: Part two" {
    helper "abcde fghij klmno pqrst fguij axcye wvxyz" "Part 1: 0\nPart 2: fgij"
}

@test "Day 02: Real" {
    helper "$(cat ./day02_input.txt)" "Part 1: 5456\nPart 2: megsdlpulxvinkatfoyzxcbvq"
}
