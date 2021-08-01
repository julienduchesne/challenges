package main

import "math"

type Point struct {
	X, Y float64
}

func (p Point) Add(other Point) Point {
	return Point{p.X + other.X, p.Y + other.Y}
}

func (p Point) Angle(other Point) float64 {
	var theta = math.Atan2(float64(other.Y-p.Y), float64(other.X-p.X))
	theta *= 180 / math.Pi
	theta += 90
	if theta < 0 {
		theta = 360 + theta
	}
	return theta
}

func (p Point) Distance(other Point) float64 {
	return math.Sqrt(math.Exp2(float64(other.X-p.X)) + math.Exp2(float64(other.Y-p.Y)))
}

func (p Point) Rotate(degrees float64) Point {
	round := func(v float64) float64 {
		return math.Round(v*1000) / 1000
	}

	degrees = degrees * math.Pi / 180
	s := math.Sin(degrees)
	c := math.Cos(degrees)
	return Point{X: round(p.X*c - p.Y*s), Y: round(p.X*s + p.Y*c)}
}
