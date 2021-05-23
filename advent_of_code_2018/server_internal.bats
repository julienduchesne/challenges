#!/usr/bin/env bats

@test "Server: Test GET" {
    REQUEST='GET /list/ HTTP/1.1
Host: localhost:8083
Content-Type: application/x-www-form-urlencoded
Accept: */*
Connection: keep-alive
User-Agent: CocoaRestClient/28 CFNetwork/1237 Darwin/20.4.0
'
    EXPECTED='HTTP/1.1 200 OK
Location: /list/

['

    response="$(printf '%s' "$REQUEST" | ./server_internal.sh)"

    echo "$response"
    [[ "$response" == "$EXPECTED"* ]]
}

@test "Server: Test POST" {
    REQUEST='POST /solve/day01.sh HTTP/1.1
Host: localhost:8083
User-Agent: CocoaRestClient/28 CFNetwork/1237 Darwin/20.4.0
Content-Type: application/x-www-form-urlencoded
Accept: */*
Content-Length: 20
Connection: keep-alive

+1
+1
-1
-2
+1
+2
+5'
    EXPECTED='HTTP/1.1 200 OK
Location: /solve/day01.sh

Part 1: 7
Part 2: 1'

    response="$(printf '%s' "$REQUEST" | ./server_internal.sh)"

    echo "$response"
    [ "$response" == "$EXPECTED" ]
}
