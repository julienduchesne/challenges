package main

func solveDay01(input string) (string, error) {
	return "hello bro" + input, nil
}

func init() {
	registerDay("Day 1: The Tyranny of the Rocket Equation", "", solveDay01)
}
