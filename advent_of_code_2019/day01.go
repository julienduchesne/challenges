package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

func solveDay01(input string) (string, error) {
	var total, totalWithFuel int
	for _, line := range strings.Split(input, "\n") {
		mass, err := strconv.Atoi(line)
		if err != nil {
			return "", err
		}
		addedOnce := false
		for {
			mass = int(math.Floor(float64(mass)/3)) - 2
			if mass < 0 {
				break
			}
			if !addedOnce {
				total += mass
				addedOnce = true
			}
			totalWithFuel += mass
		}
	}

	return fmt.Sprintf("Part 1: %d\nPart 2: %d", total, totalWithFuel), nil
}

func init() {
	registerDay("Day 1: The Tyranny of the Rocket Equation", "", solveDay01)
}
