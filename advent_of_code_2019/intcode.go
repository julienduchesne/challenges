// Used in days 2, 5, 7

package main

func runIntcode(codeValues []int, inputChannel chan int, outputChannel chan int) ([]int, int) {
	code := make([]int, len(codeValues))
	copy(code, codeValues)
	position := 0
	var outputs []int
outer:
	for {
		v := code[position]
		opcode := v % 100
		firstImmediate := (v%1000)/100 == 1
		secondImmediate := (v%10000)/1000 == 1
		var first, second, third int

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
			if !firstImmediate {
				first = code[first]
			}
			if !secondImmediate {
				second = code[second]
			}

			if opcode == 1 {
				code[third] = first + second
			} else if opcode == 2 {
				code[third] = first * second
			}
			position += 4
		case 3:
			code[first] = <-inputChannel
			position += 2
		case 4:
			if !firstImmediate {
				first = code[first]
			}
			if outputChannel != nil {
				outputChannel <- first
			}
			outputs = append(outputs, first)
			position += 2
		case 5, 6:
			if !firstImmediate {
				first = code[first]
			}
			if !secondImmediate {
				second = code[second]
			}
			if (first != 0 && opcode == 5) || (first == 0 && opcode == 6) {
				position = second
			} else {
				position += 3
			}
		case 7, 8:
			if !firstImmediate {
				first = code[first]
			}
			if !secondImmediate {
				second = code[second]
			}
			if (first < second && opcode == 7) || (first == second && opcode == 8) {
				code[third] = 1
			} else {
				code[third] = 0
			}
			position += 4
		case 99:
			break outer
		}
		position = position % len(code)
	}
	output := -1
	if len(outputs) > 0 {
		output = outputs[len(outputs)-1]
	}
	return code, output
}
