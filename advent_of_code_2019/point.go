package main

import "math"

type Point struct {
	X, Y int
}

func (a Point) Angle(other Point) float64 {
	var theta = math.Atan2(float64(other.Y-a.Y), float64(other.X-a.X))
	theta *= 180 / math.Pi
	theta += 90
	if theta < 0 {
		theta = 360 + theta
	}
	return theta
}

func (a Point) Distance(other Point) float64 {
	return math.Sqrt(math.Exp2(float64(other.X-a.X)) + math.Exp2(float64(other.Y-a.Y)))
}
