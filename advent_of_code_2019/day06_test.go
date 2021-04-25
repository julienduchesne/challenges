package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSolveDay6(t *testing.T) {
	var tests = []struct {
		input    string
		expected string
	}{
		{`COM)B
		B)C
		C)D
		D)E
		E)F
		B)G
		G)H
		D)I
		E)J
		J)K
		K)L`, "Part 1: 42\nPart 2: -1"},
		{`COM)B
		B)C
		C)D
		D)E
		E)F
		B)G
		G)H
		D)I
		E)J
		J)K
		K)L
		K)YOU
		I)SAN`, "Part 1: 54\nPart 2: 4"},
	}
	for _, test := range tests {
		solved, err := solveDay06(test.input)
		assert.NoError(t, err)
		assert.Equal(t, test.expected, solved, test.input)
	}
}
