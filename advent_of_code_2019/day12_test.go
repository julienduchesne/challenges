package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSolveDay12(t *testing.T) {
	var testcases = []struct {
		input  string
		output string
	}{
		{
			`<x=-1, y=0, z=2>
			<x=2, y=-10, z=-7>
			<x=4, y=-8, z=8>
			<x=3, y=5, z=-1>`,
			"Part 1: 183\nPart 2: 2772",
		},
		{
			`<x=-4, y=-14, z=8>
			<x=1, y=-8, z=10>
			<x=-15, y=2, z=1>
			<x=-17, y=-17, z=16>`,
			"Part 1: 12466\nPart 2: 360689156787864",
		},
	}
	for _, tc := range testcases {
		solved, err := solveDay12(tc.input)
		assert.NoError(t, err)
		assert.Equal(t, tc.output, solved)
	}
}
