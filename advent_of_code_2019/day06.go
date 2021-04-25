package main

import (
	"fmt"
	"strings"

	"github.com/scylladb/go-set/strset"
)

func getOrbitDistance(pointA string, pointB string, directOrbits map[string]string, cache *map[string]*strset.Set) int {
	if _, ok := directOrbits[pointA]; !ok {
		return -1
	}
	if _, ok := directOrbits[pointB]; !ok {
		return -1
	}

	pointAOrbits := getOrbits(pointA, directOrbits, cache)
	pointBOrbits := getOrbits(pointB, directOrbits, cache)

	commonOrbits := strset.Intersection(pointAOrbits, pointBOrbits)
	if commonOrbits.Size() == 0 {
		return -1
	}

	min := 100000000
	commonOrbits.Each(func(common string) bool {
		distance := 0
		point := pointA
		for point != common {
			point = directOrbits[point]
			distance += 1
		}
		point = pointB
		for point != common {
			point = directOrbits[point]
			distance += 1
		}
		if distance < min {
			min = distance
		}
		return true
	})

	// -2 to account for the fact that we don't want to move on pointB, we only want to orbit the same celestial body
	return min - 2
}

func getOrbits(orbit string, directOrbits map[string]string, cache *map[string]*strset.Set) *strset.Set {
	if value, ok := (*cache)[orbit]; ok {
		return value
	}
	center, ok := directOrbits[orbit]
	if !ok {
		return strset.New()
	}

	set := strset.New(center)
	set.Merge(getOrbits(center, directOrbits, cache))
	(*cache)[orbit] = set
	return set
}

func countOrbits(directOrbits map[string]string) (int, *map[string]*strset.Set) {
	indirectOrbits := map[string]*strset.Set{}
	total := 0
	for _, center := range directOrbits {
		total += 1
		total += getOrbits(center, directOrbits, &indirectOrbits).Size()
	}
	return total, &indirectOrbits
}

func solveDay06(input string) (string, error) {
	directOrbits := map[string]string{}

	for _, line := range strings.Split(input, "\n") {
		split := strings.Split(strings.TrimSpace(line), ")")
		center, orbit := split[0], split[1]
		directOrbits[orbit] = center
	}

	orbitCount, indirectOrbits := countOrbits(directOrbits)
	distance := getOrbitDistance("YOU", "SAN", directOrbits, indirectOrbits)

	return fmt.Sprintf("Part 1: %d\nPart 2: %d", orbitCount, distance), nil
}

func init() {
	registerDay("Day 6: Universal Orbit Map", "", solveDay06)
}
