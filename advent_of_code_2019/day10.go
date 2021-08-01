package main

import (
	"fmt"
	"sort"
	"strings"
)

func solveDay10(input string) (string, error) {
	var asteroids []Point
	for y, line := range strings.Split(input, "\n") {
		line = strings.TrimSpace(line)
		for x, char := range line {
			if char == '#' {
				asteroids = append(asteroids, Point{X: float64(x), Y: float64(y)})
			}
		}
	}

	var (
		p1Index    int
		p1SightMap map[float64][]Point
	)
	for i, asteroid := range asteroids {
		sightMap := map[float64][]Point{}
		for j, other := range asteroids {
			if i == j {
				continue
			}
			sightMap[asteroid.Angle(other)] = append(sightMap[asteroid.Angle(other)], other)
		}
		if len(sightMap) > len(p1SightMap) {
			p1Index = i
			p1SightMap = sightMap
		}
	}
	p1 := len(p1SightMap)

	p2 := float64(-1)
	p1Asteroid := asteroids[p1Index]
	keys := []float64{}
	for key, values := range p1SightMap {
		keys = append(keys, key)
		sort.Slice(p1SightMap[key][:], func(i, j int) bool {
			return p1Asteroid.Distance(values[i]) > p1Asteroid.Distance(values[j])
		})
	}
	sort.Float64s(keys)

	var countBefore, countAfter int
	p2Asteroids := []Point{}
	for {
		countBefore = len(p2Asteroids)
		if countBefore >= 200 || (countBefore != 0 && countBefore == countAfter) {
			break
		}
		for _, key := range keys {
			if len(p1SightMap[key]) > 0 {
				p2Asteroids = append(p2Asteroids, p1SightMap[key][0])
				p1SightMap[key] = p1SightMap[key][1:]

			}
		}
		countAfter = len(p2Asteroids)
	}

	if len(p2Asteroids) >= 200 {
		p2A := p2Asteroids[199]
		p2 = p2A.X*100 + p2A.Y
	}

	return fmt.Sprintf("Part 1: %d\nPart 2: %d", p1, int(p2)), nil
}

func init() {
	registerDay("Day 10: Monitoring Station", "", solveDay10)
}
