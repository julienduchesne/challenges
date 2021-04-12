package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"strconv"
)

type Day struct {
	ID          int    `json:"id"`
	Title       string `json:"title"`
	Description string `json:"description"`
	solveFunc   func(string) (string, error)
}

var days []Day

func registerDay(title, description string, solveFunc func(string) (string, error)) {
	days = append(days, Day{ID: len(days) + 1, Title: title, Description: description, solveFunc: solveFunc})
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("No command passed")
		os.Exit(1)
	}

	if command := os.Args[1]; command == "list" {
		marshalled, err := json.MarshalIndent(days, "", "  ")
		if err != nil {
			fmt.Printf("Got an error while converting challenges to JSON: %v", err)
			os.Exit(1)
		}
		fmt.Println(string(marshalled))
	} else if command == "solve" {
		if len(os.Args) < 3 {
			fmt.Println("Expected and ID for the solve comand")
			os.Exit(1)
		}
		id, err := strconv.Atoi(os.Args[2])
		if err != nil {
			fmt.Printf("Got an error while converting the solve ID (%s) to an integer: %v\n", os.Args[2], err)
			os.Exit(1)
		}

		var day *Day
		for _, d := range days {
			if d.ID == id {
				day = &d
			}
		}
		if day == nil {
			fmt.Printf("ID %d did not match a day\n", id)
			os.Exit(1)
		}

		input := ""
		if len(os.Args) >= 4 {
			input = os.Args[3]
		} else if stat, _ := os.Stdin.Stat(); (stat.Mode() & os.ModeCharDevice) == 0 {
			scanner := bufio.NewScanner(os.Stdin)
			for scanner.Scan() {
				input += scanner.Text() + "\n"
			}
		}

		solved, err := day.solveFunc(input)
		if err != nil {
			fmt.Printf("Got an error while solving: %v\n", err)
			os.Exit(1)
		}
		fmt.Println(solved)

	} else {
		fmt.Printf("Unexpected command: %s\n", command)
		os.Exit(1)
	}
}
