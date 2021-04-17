package main

import (
	"fmt"
	"strconv"
	"strings"
)

func tryNounVerb(codeValues []int, noun, verb int) int {
	code := make([]int, len(codeValues))
	copy(code, codeValues)
	position := 0
	code[1] = noun
	code[2] = verb

outer:
	for {
		switch v := code[position]; v {
		case 1, 2:
			first, second, third := code[position+1], code[position+2], code[position+3]
			if v == 1 {
				code[third] = code[first] + code[second]
			} else if v == 2 {
				code[third] = code[first] * code[second]
			}
		case 99:
			break outer
		}
		position = (position + 4) % len(code)
	}
	return code[0]
}

func solveDay02(input string) (string, error) {
	var code []int
	for _, v := range strings.Split(input, ",") {
		vInt, err := strconv.Atoi(v)
		if err != nil {
			return "", err
		}
		code = append(code, vInt)
	}

	part2 := 0
outer:
	for i := 1; i <= 20; i++ {
		for j := 1; j <= 20; j++ {
			if tryNounVerb(code[:], i, j) == 19690720 {
				part2 = i*100 + j
				break outer
			}
		}
	}

	return fmt.Sprintf("Part 1: %d\nPart 2: %d", tryNounVerb(code[:], 12, 2), part2), nil
}

func init() {
	registerDay("Day 2: 1202 Program Alarm", "", solveDay02)
}
