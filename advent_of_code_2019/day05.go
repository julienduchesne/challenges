package main

import (
	"fmt"
	"strconv"
	"strings"
)

func solveDay05(input string) (string, error) {
	var code []int
	for _, v := range strings.Split(input, ",") {
		vInt, err := strconv.Atoi(v)
		if err != nil {
			return "", err
		}
		code = append(code, vInt)
	}
	_, p1 := runIntcode(code, 1)
	_, p2 := runIntcode(code, 5)
	return fmt.Sprintf("Part 1: %d\nPart 2: %d", p1, p2), nil
}

func init() {
	registerDay("Day 5: Sunny with a Chance of Asteroids", "", solveDay05)
}
