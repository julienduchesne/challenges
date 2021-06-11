#!/usr/bin/env bats

helper() {
    result="$(echo "$1" | ./day04.sh)"
    echo "Result: $result"
    [ "$result" == "$(echo -e "$2")" ]
}

@test "Day 04: Simple" {
    helper "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up" "Part 1: 240\nPart 2: 4455"
}


@test "Day 04: Real" {
    helper "$(cat ./day04_input.txt)" "Part 1: 101194\nPart 2: 0"
}
