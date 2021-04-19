package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSolveDay4(t *testing.T) {
	var tests = []struct {
		input    string
		expected string
	}{
		{"111111-111111", "Part 1: 1\nPart 2: 0"},
		{"223450-223450", "Part 1: 0\nPart 2: 0"},
		{"123789-123789", "Part 1: 0\nPart 2: 0"},
		{"112233-112233", "Part 1: 1\nPart 2: 1"},
		{"123444-123444", "Part 1: 1\nPart 2: 0"},
		{"111122-111122", "Part 1: 1\nPart 2: 1"},
		{"248345-746315", "Part 1: 1019\nPart 2: 660"},
	}
	for _, test := range tests {
		solved, err := solveDay04(test.input)
		assert.NoError(t, err)
		assert.Equal(t, test.expected, solved, test.input)
	}

}
