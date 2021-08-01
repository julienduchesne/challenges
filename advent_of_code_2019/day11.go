package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

func solveDay11(input string) (string, error) {
	var code []int
	for _, v := range strings.Split(input, ",") {
		vInt, err := strconv.Atoi(v)
		if err != nil {
			return "", err
		}
		code = append(code, vInt)
	}

	run := func(startChar int) map[Point]int {
		outputChan := make(chan int)
		inputChan := make(chan int, 1)
		go runIntcode(code, inputChan, outputChan)

		inputChan <- startChar
		currentPosition := Point{X: 0, Y: 0}
		currentDirection := Point{X: 0, Y: -1}
		points := map[Point]int{currentPosition: startChar}
		color := -1
		for v := range outputChan {
			if color == -1 {
				color = v
				points[currentPosition] = color
				continue
			}

			if v == 0 {
				currentDirection = currentDirection.Rotate(-90)
			} else {
				currentDirection = currentDirection.Rotate(90)
			}
			color = -1
			currentPosition = currentPosition.Add(currentDirection)
			if len(inputChan) == 0 {
				inputChan <- points[currentPosition]
			}
		}
		return points
	}

	p2Points := run(1)
	minX, maxX, minY, maxY := 0.0, 0.0, 0.0, 0.0
	for p := range p2Points {
		minX = math.Min(minX, p.X)
		maxX = math.Max(maxX, p.X)
		minY = math.Min(minY, p.Y)
		maxY = math.Max(maxY, p.Y)
	}

	builder := strings.Builder{}
	for y := minY; y <= maxY; y++ {
		builder.WriteString(fmt.Sprintf("%3d ", int(y)))
		for x := minX; x < maxX; x++ {
			if p2Points[Point{X: x, Y: y}] == 1 {
				builder.WriteString("■")
			} else {
				builder.WriteString(" ")
			}
		}
		builder.WriteString("\n")
	}

	return fmt.Sprintf("Part 1: %d\nPart 2:\n%s", len(run(0)), builder.String()), nil
}

func init() {
	registerDay("Day 11: Space Police", "", solveDay11)
}
