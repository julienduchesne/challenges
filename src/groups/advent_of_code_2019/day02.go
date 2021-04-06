package main

func solveDay02(input string) (string, error) {
	return "hello" + input, nil
}

func init() {
	registerDay("Day 2: 1202 Program Alarm", "", solveDay02)
}
