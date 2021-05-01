package main

import (
	"fmt"
	"strconv"
	"strings"
)

func tryPhaseSetting(code []int, settings []int, loop bool) int {
	input := 0
	if loop {
		// Run computers as goroutines with channels to communicate between them
		channels := []chan int{}
		for _, setting := range settings {
			inputs := make(chan int, 1)
			inputs <- setting
			channels = append(channels, inputs)
		}
		for i := 0; i < len(channels)-1; i++ {
			inputChannel := channels[i]
			outputChannel := channels[(i+1)%len(channels)]
			go runIntcode(code, inputChannel, outputChannel)
		}
		channels[0] <- input
		i := len(channels) - 1
		inputChannel := channels[i]
		outputChannel := channels[(i+1)%len(channels)]
		_, input = runIntcode(code, inputChannel, outputChannel)
	} else {
		for _, setting := range settings {
			inputs := make(chan int, 100)
			inputs <- setting
			inputs <- input
			// Enter zeroes for cases that require more than one input
			for x := 0; x < 98; x++ {
				inputs <- 0
			}
			_, input = runIntcode(code, inputs, nil)
		}
	}

	return input
}

func findBestSettings(code []int, settingsNumbers []int, loop bool) ([]int, int) {
	max := 0
	var maxSetting []int
	for _, settings := range permutations(settingsNumbers) {
		if value := tryPhaseSetting(code, settings, loop); value > max {
			max = value
			maxSetting = settings
		}
	}
	return maxSetting, max
}

func solveDay07(input string) (string, error) {
	var code []int
	for _, v := range strings.Split(input, ",") {
		vInt, err := strconv.Atoi(v)
		if err != nil {
			return "", err
		}
		code = append(code, vInt)
	}
	_, p1 := findBestSettings(code, []int{0, 1, 2, 3, 4}, false)
	_, p2 := findBestSettings(code, []int{5, 6, 7, 8, 9}, true)
	return fmt.Sprintf("Part 1: %d\nPart 2: %d", p1, p2), nil
}

func init() {
	registerDay("Day 7: Amplification Circuit", "", solveDay07)
}
