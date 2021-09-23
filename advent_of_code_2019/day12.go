package main

import (
	"crypto/sha256"
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Moon struct {
	PosX, PosY, PosZ int
	VelX, VelY, VelZ int
}

type Moons []*Moon

func (moons Moons) ApplyGravity() {
	for _, moon := range moons {
		for _, other := range moons {
			moon.VelX -= compare(moon.PosX, other.PosX)
			moon.VelY -= compare(moon.PosY, other.PosY)
			moon.VelZ -= compare(moon.PosZ, other.PosZ)
		}
	}

	for _, moon := range moons {
		moon.PosX += moon.VelX
		moon.PosY += moon.VelY
		moon.PosZ += moon.VelZ
	}
}

func (moons Moons) Energy() int {
	total := 0
	for _, moon := range moons {
		potential := math.Abs(float64(moon.PosX)) + math.Abs(float64(moon.PosY)) + math.Abs(float64(moon.PosZ))
		kinetic := math.Abs(float64(moon.VelX)) + math.Abs(float64(moon.VelY)) + math.Abs(float64(moon.VelZ))
		total += int(potential * kinetic)
	}
	return total
}

func (moons Moons) Hashes() (string, string, string) {
	hashX, hashY, hashZ := sha256.New(), sha256.New(), sha256.New()
	for _, moon := range moons {
		hashX.Write([]byte(fmt.Sprint(moon.PosX, "-", moon.VelX)))
		hashY.Write([]byte(fmt.Sprint(moon.PosY, "-", moon.VelY)))
		hashZ.Write([]byte(fmt.Sprint(moon.PosZ, "-", moon.VelZ)))
	}

	return string(hashX.Sum(nil)), string(hashY.Sum(nil)), string(hashZ.Sum(nil))
}

func solveDay12(input string) (string, error) {
	var moons Moons = []*Moon{}
	for _, line := range strings.Split(input, "\n") {
		moon := &Moon{}
		line = strings.TrimSpace(line)
		line = strings.Trim(line, "<>")
		parts := strings.Split(line, ",")
		for _, coord := range parts {
			parts := strings.Split(coord, "=")
			letter, numberStr := parts[0], parts[1]
			number, err := strconv.Atoi(numberStr)
			if err != nil {
				return "", err
			}
			if strings.ContainsRune(letter, 'x') {
				moon.PosX = number
			} else if strings.ContainsRune(letter, 'y') {
				moon.PosY = number
			} else if strings.ContainsRune(letter, 'z') {
				moon.PosZ = number
			}
		}
		moons = append(moons, moon)
	}

	var part1, part2 int
	var indexX, indexY, indexZ int
	var seenX, seenY, seenZ = make(map[string]int), make(map[string]int), make(map[string]int)
	for i := 1; i <= 300000; i++ {
		moons.ApplyGravity()
		if i == 1000 {
			part1 = moons.Energy()
		}
		hashX, hashY, hashZ := moons.Hashes()
		if indexX == 0 {
			if _, ok := seenX[hashX]; ok {
				indexX = i - 1
			}
		}
		if indexY == 0 {
			if _, ok := seenY[hashY]; ok {
				indexY = i - 1
			}
		}
		if indexZ == 0 {
			if _, ok := seenZ[hashZ]; ok {
				indexZ = i - 1
			}
		}
		seenX[hashX] = i
		seenY[hashY] = i
		seenZ[hashZ] = i

		if indexX != 0 && indexY != 0 && indexZ != 0 {
			part2 = lcm(indexX, indexY, indexZ)
		}
		if part1 != 0 && part2 != 0 {
			break
		}
	}
	return fmt.Sprintf("Part 1: %d\nPart 2: %d", part1, part2), nil
}

func init() {
	registerDay("Day 12: The N-Body Problem", "", solveDay12)
}
