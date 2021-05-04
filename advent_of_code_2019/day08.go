package main

import (
	"fmt"
	"strings"
)

type TwoDim [][]int

func NewTwoDim(x, y int) TwoDim {
	array := make([][]int, y)
	for i := range array {
		array[i] = make([]int, x)
	}
	return array
}

func (a *TwoDim) Count(v int) int {
	count := 0
	for _, row := range *a {
		for _, digit := range row {
			if digit == v {
				count++
			}
		}
	}
	return count
}

func solveDay08WithSize(input string, xSize, ySize int) (string, error) {
	layers := []TwoDim{}
	rowIndex := 0
	layerIndex := 0

	for _, digit := range input {
		if len(layers) == layerIndex {
			layers = append(layers, TwoDim{})
		}
		layer := &layers[layerIndex]
		if len(*layer) == rowIndex {
			*layer = append(*layer, []int{})
		}
		row := &((*layer)[rowIndex])

		*row = append(*row, int(digit-'0'))

		if rowIndex == ySize-1 && len(*row) == xSize {
			layerIndex += 1
			rowIndex = 0
		} else if len(*row) == xSize {
			rowIndex += 1
		}
	}

	min := 99999999
	minIndex := -1
	for i, layer := range layers {
		if count := layer.Count(0); count < min {
			min = count
			minIndex = i
		}
	}

	resultLayer := NewTwoDim(xSize, ySize)
	for x := 0; x < xSize; x++ {
		for y := 0; y < ySize; y++ {
			for _, layer := range layers {
				if v := layer[y][x]; v == 0 || v == 1 {
					resultLayer[y][x] = v
					break
				}
			}
		}
	}

	result := ""
	for _, row := range resultLayer {
		for _, digit := range row {
			if digit == 1 {
				result += "█"
			} else {
				result += " "
			}
		}
		result = strings.TrimSpace(result) + "\n"
	}

	p1Layer := layers[minIndex]
	return fmt.Sprintf("Part 1: %d\nPart 2:\n%s", p1Layer.Count(1)*p1Layer.Count(2), strings.TrimSpace(result)), nil
}

func solveDay08(input string) (string, error) {
	return solveDay08WithSize(input, 25, 6)

}

func init() {
	registerDay("Day 8: Space Image Format", "", solveDay08)
}
