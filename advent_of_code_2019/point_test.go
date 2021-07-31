package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPointAngle(t *testing.T) {
	var testcases = []struct {
		center, other Point
		expected      float64
	}{
		{
			Point{X: 10, Y: 10},
			Point{X: 10, Y: 9},
			0.0,
		},
		{
			Point{X: 10, Y: 10},
			Point{X: 11, Y: 10},
			90.0,
		},
		{
			Point{X: 10, Y: 10},
			Point{X: 10, Y: 11},
			180.0,
		},
		{
			Point{X: 10, Y: 10},
			Point{X: 9, Y: 10},
			270.0,
		},
	}
	for _, tc := range testcases {
		assert.Equal(t, tc.expected, tc.center.Angle(tc.other))
	}
}
