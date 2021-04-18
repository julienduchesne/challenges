package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"

	"github.com/juliangruber/go-intersect"
)

type coordinate struct {
	x, y int
}

func getCoordinates(input string) (map[coordinate]int, error) {
	steps := 0
	currentX := 0
	currentY := 0

	var coords map[coordinate]int = make(map[coordinate]int)
	for _, stepStr := range strings.Split(input, ",") {
		stepStr = strings.TrimSpace(stepStr)
		d, err := strconv.Atoi(stepStr[1:])
		if err != nil {
			return nil, fmt.Errorf("got an error while parsing %s: %v", stepStr, err)
		}
		addCoord := func() {
			steps++
			newCoord := coordinate{x: currentX, y: currentY}
			if _, ok := coords[newCoord]; !ok {
				coords[newCoord] = steps
			}
		}
		switch stepStr[0] {
		case 'R':
			for x := 0; x < d; x++ {
				currentX++
				addCoord()
			}
		case 'D':
			for y := 0; y < d; y++ {
				currentY -= 1
				addCoord()
			}
		case 'L':
			for x := 0; x < d; x++ {
				currentX -= 1
				addCoord()
			}
		case 'U':
			for y := 0; y < d; y++ {
				currentY += 1
				addCoord()
			}
		}
	}
	return coords, nil
}

func getIntersectingCoords(map1 map[coordinate]int, map2 map[coordinate]int) []coordinate {
	getMapKeys := func(m map[coordinate]int) []coordinate {
		keys := make([]coordinate, 0, len(m))
		for k := range m {
			keys = append(keys, k)
		}
		return keys
	}

	var coords []coordinate
	crosses := intersect.Hash(getMapKeys(map1), getMapKeys(map2)).([]interface{})
	for _, cross := range crosses {
		crossCoord := cross.(coordinate)
		coords = append(coords, crossCoord)
	}
	return coords
}

func solveDay03(input string) (string, error) {
	var (
		wire1, wire2 map[coordinate]int
		err          error
	)
	split := strings.Split(strings.TrimSpace(input), "\n")
	if wire1, err = getCoordinates(split[0]); err != nil {
		return "", err
	}
	if wire2, err = getCoordinates(split[1]); err != nil {
		return "", err
	}

	shortestDistance := 10000000000
	leastSteps := 10000000000
	for _, cross := range getIntersectingCoords(wire1, wire2) {
		shortestDistance = min(shortestDistance, int(math.Abs(float64(cross.x)))+int(math.Abs(float64(cross.y))))
		leastSteps = min(leastSteps, wire1[cross]+wire2[cross])
	}

	return fmt.Sprintf("Part 1: %d\nPart 2: %d", shortestDistance, leastSteps), nil
}

func init() {
	registerDay("Day 3: Crossed Wires", "", solveDay03)
}
