#!/usr/bin/env bats

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
source ./lib/string.sh

trim_helper() {
    result="$(trim $1)"
    expected="$2"
    echo "Result '$result'. Expected: '$expected'"
    [ "$result" == "$expected" ]
}

@test "trim(): No space" {
    trim_helper "test" "test"
}

@test "trim(): Spaces at end and beginning" {
    trim_helper " test   " "test"
}

@test "trim(): Newlines" {
    trim_helper "$(echo -e " test \n")" "test"
}