package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSolveDay1(t *testing.T) {
	var tests = []struct {
		input    string
		expected string
	}{
		{"14", `Part 1: 2
Part 2: 2`},
		{"1969", `Part 1: 654
Part 2: 966`},
		{"100756", `Part 1: 33583
Part 2: 50346`},
	}
	for _, test := range tests {
		solved, err := solveDay01(test.input)
		assert.NoError(t, err)
		assert.Equal(t, test.expected, solved)
	}

}
