package main

import (
	"fmt"
	"strconv"
	"strings"
)

func solveDay09(input string) (string, error) {
	var code []int
	for _, v := range strings.Split(input, ",") {
		vInt, err := strconv.Atoi(v)
		if err != nil {
			return "", err
		}
		code = append(code, vInt)
	}
	p1Input := make(chan int, 1)
	p1Input <- 1
	_, p1 := runIntcode(code, p1Input, nil)
	p2Input := make(chan int, 1)
	p2Input <- 2
	_, p2 := runIntcode(code, p2Input, nil)

	return fmt.Sprintf("Part 1: %d\nPart 2: %d", p1, p2), nil

}

func init() {
	registerDay("Day 9: Sensor Boost", "", solveDay09)
}
