#!/usr/bin/env bats

helper() {
    result="$(echo "$1" | ./day03.sh)"
    echo "Result: $result"
    [ "$result" == "$(echo -e "$2")" ]
}

@test "Day 03: Simple" {
    helper "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2" "Part 1: 4\nPart 2: #3"
}


@test "Day 03: Real" {
    helper "$(cat ./day03_input.txt)" "Part 1: 110195\nPart 2: megsdlpulxvinkatfoyzxcbvq"
}
