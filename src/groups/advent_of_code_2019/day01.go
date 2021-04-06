package main

func solveDay01(input string) (string, error) {
	return "hello1" + input, nil
}

func init() {
	registerDay("Day 1: The Tyranny of the Rocket Equation", "", solveDay01)
}
