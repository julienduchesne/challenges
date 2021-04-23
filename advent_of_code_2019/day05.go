package main

import (
	"fmt"
	"strconv"
	"strings"
)

func solveDay05Case(codeValues []int, inputValue int) int {
	code := make([]int, len(codeValues))
	copy(code, codeValues)
	position := 0
	var outputs []int
outer:
	for {
		v := code[position]
		opcode := v % 100
		firstImmediate := (v%1000)/100 == 1
		secondImmediate := (v%10000)/1000 == 1
		var first, second, third int

		if len(code) > position+1 {
			first = code[position+1]
		}
		if len(code) > position+2 {
			second = code[position+2]
		}
		if len(code) > position+3 {
			third = code[position+3]
		}

		switch opcode {
		case 1, 2:
			if !firstImmediate {
				first = code[first]
			}
			if !secondImmediate {
				second = code[second]
			}

			if opcode == 1 {
				code[third] = first + second
			} else if opcode == 2 {
				code[third] = first * second
			}
			position += 4
		case 3:
			code[first] = inputValue
			position += 2
		case 4:
			if !firstImmediate {
				first = code[first]
			}
			outputs = append(outputs, first)
			position += 2
		case 5, 6:
			if !firstImmediate {
				first = code[first]
			}
			if !secondImmediate {
				second = code[second]
			}
			if (first != 0 && opcode == 5) || (first == 0 && opcode == 6) {
				position = second
			} else {
				position += 3
			}
		case 7, 8:
			if !firstImmediate {
				first = code[first]
			}
			if !secondImmediate {
				second = code[second]
			}
			if (first < second && opcode == 7) || (first == second && opcode == 8) {
				code[third] = 1
			} else {
				code[third] = 0
			}
			position += 4
		case 99:
			break outer
		}
		position = position % len(code)
	}
	return outputs[len(outputs)-1]
}

func solveDay05(input string) (string, error) {
	var code []int
	for _, v := range strings.Split(input, ",") {
		vInt, err := strconv.Atoi(v)
		if err != nil {
			return "", err
		}
		code = append(code, vInt)
	}
	return fmt.Sprintf("Part 1: %d\nPart 2: %d", solveDay05Case(code, 1), solveDay05Case(code, 5)), nil
}

func init() {
	registerDay("Day 5: Sunny with a Chance of Asteroids", "", solveDay05)
}
