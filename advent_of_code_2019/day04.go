package main

import (
	"fmt"
	"strconv"
	"strings"
)

func isValidPassword(n int) (bool, bool) {
	matchesFirst := false
	matchesSecond := false
	nStr := fmt.Sprint(n)
	last := nStr[0]
	count := 1
	for i := 1; i < len(nStr); i++ {
		val := nStr[i]
		if val < last {
			return false, false
		}
		if val == last {
			matchesFirst = true
			count++
		} else {
			if count == 2 {
				matchesSecond = true
			}
			count = 1
		}
		last = val
	}

	return matchesFirst, count == 2 || matchesSecond
}

func solveDay04(input string) (string, error) {
	split := strings.Split(input, "-")
	start, err := strconv.Atoi(split[0])
	if err != nil {
		return "", err
	}
	end, err := strconv.Atoi(split[1])
	if err != nil {
		return "", err
	}

	resultChan := make(chan struct {
		first  bool
		second bool
	})
	for x := start; x <= end; x++ {
		n := x
		go func() {
			firstResult, secondResult := isValidPassword(n)
			resultChan <- struct {
				first  bool
				second bool
			}{firstResult, secondResult}
		}()
	}

	var firstTotal, secondTotal int
	for x := start; x <= end; x++ {
		result := <-resultChan
		if result.first {
			firstTotal++
		}
		if result.second {
			secondTotal++
		}
	}

	return fmt.Sprintf("Part 1: %d\nPart 2: %d", firstTotal, secondTotal), nil
}

func init() {
	registerDay("Day 4: Secure Container", "", solveDay04)
}
