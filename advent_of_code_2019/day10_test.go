package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAsteroidAngle(t *testing.T) {
	var testcases = []struct {
		center, other Asteroid
		expected      float64
	}{
		{
			Asteroid{X: 10, Y: 10},
			Asteroid{X: 10, Y: 9},
			0.0,
		},
		{
			Asteroid{X: 10, Y: 10},
			Asteroid{X: 11, Y: 10},
			90.0,
		},
		{
			Asteroid{X: 10, Y: 10},
			Asteroid{X: 10, Y: 11},
			180.0,
		},
		{
			Asteroid{X: 10, Y: 10},
			Asteroid{X: 9, Y: 10},
			270.0,
		},
	}
	for _, tc := range testcases {
		assert.Equal(t, tc.expected, tc.center.Angle(tc.other))
	}
}

func TestSolveDay10(t *testing.T) {
	var testcases = []struct {
		input  string
		output string
	}{
		// {`.#..#
		// .....
		// #####
		// ....#
		// ...##`, "Part 1: 8\nPart 2: -1"},

		// {`......#.#.
		// #..#.#....
		// ..#######.
		// .#.#.###..
		// .#..#.....
		// ..#....#.#
		// #..#....#.
		// .##.#..###
		// ##...#..#.
		// .#....####`, "Part 1: 33\nPart 2: -1"},

		// {`#.#...#.#.
		// .###....#.
		// .#....#...
		// ##.#.#.#.#
		// ....#.#.#.
		// .##..###.#
		// ..#...##..
		// ..##....##
		// ......#...
		// .####.###.`, "Part 1: 35\nPart 2: -1"},

		// {`.#..#..###
		// ####.###.#
		// ....###.#.
		// ..###.##.#
		// ##.##.#.#.
		// ....###..#
		// ..#.#..#.#
		// #..#.#.###
		// .##...##.#
		// .....#.#..`, "Part 1: 41\nPart 2: -1"},

		{`.#..##.###...#######
		##.############..##.
		.#.######.########.#
		.###.#######.####.#.
		#####.##.#.##.###.##
		..#####..#.#########
		####################
		#.####....###.#.#.##
		##.#################
		#####.##.###..####..
		..######..##.#######
		####.##.####...##..#
		.#####..#.######.###
		##...#.##########...
		#.##########.#######
		.####.#.###.###.#.##
		....##.##.###..#####
		.#.#.###########.###
		#.#.#.#####.####.###
		###.##.####.##.#..##`, "Part 1: 210\nPart 2: 802"},
	}
	for _, tc := range testcases {
		solved, err := solveDay10(tc.input)
		assert.NoError(t, err)
		assert.Equal(t, tc.output, solved)
	}
}
