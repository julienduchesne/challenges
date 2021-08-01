// Used in days 2, 5, 7, 9, 11

package main

func runIntcode(codeValues []int, inputChannel chan int, outputChannel chan int) ([]int, int) {
	code := make([]int, len(codeValues))
	copy(code, codeValues)

	writeValue := func(pos, v int) {
		// If the slice is not long enough, expand it
		for i := len(code); i <= pos; i++ {
			code = append(code, 0)
		}
		code[pos] = v
	}

	position := 0
	relativeBase := 0
	var outputs []int
outer:
	for {
		v := code[position]
		opcode := v % 100
		firstMode := (v % 1000) / 100
		secondMode := (v % 10000) / 1000
		thirdMode := (v % 100000) / 10000
		var first, second, third int

		getRelOrAbs := func(v int, mode int) int {
			if mode == 0 || mode == 2 {
				pos := v
				if mode == 2 {
					pos = relativeBase + v
				}
				// If the slice is not long enough, expand it
				for i := len(code); i <= pos; i++ {
					code = append(code, 0)
				}
				return code[pos]
			}
			return v
		}

		if len(code) > position+1 {
			first = code[position+1]
		}
		if len(code) > position+2 {
			second = code[position+2]
		}
		if len(code) > position+3 {
			third = code[position+3]
		}

		switch opcode {
		case 1, 2:
			first = getRelOrAbs(first, firstMode)
			second = getRelOrAbs(second, secondMode)
			if thirdMode == 2 {
				third += relativeBase
			}

			if opcode == 1 {
				writeValue(third, first+second)
			} else if opcode == 2 {
				writeValue(third, first*second)
			}
			position += 4
		case 3:
			if firstMode == 2 {
				first += relativeBase
			}
			writeValue(first, <-inputChannel)
			position += 2
		case 4:
			first = getRelOrAbs(first, firstMode)
			if outputChannel != nil {
				outputChannel <- first
			}
			outputs = append(outputs, first)
			position += 2
		case 5, 6:
			first = getRelOrAbs(first, firstMode)
			second = getRelOrAbs(second, secondMode)

			if (first != 0 && opcode == 5) || (first == 0 && opcode == 6) {
				position = second
			} else {
				position += 3
			}
		case 7, 8:
			first = getRelOrAbs(first, firstMode)
			second = getRelOrAbs(second, secondMode)
			if thirdMode == 2 {
				third += relativeBase
			}

			if (first < second && opcode == 7) || (first == second && opcode == 8) {
				writeValue(third, 1)
			} else {
				writeValue(third, 0)
			}
			position += 4
		case 9:
			relativeBase += getRelOrAbs(first, firstMode)
			position += 2
		case 99:
			break outer
		}
		position = position % len(code)
	}
	output := -1
	if len(outputs) > 0 {
		output = outputs[len(outputs)-1]
	}

	if outputChannel != nil {
		close(outputChannel)
	}

	return code, output
}
