package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
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

func list(w http.ResponseWriter, r *http.Request) {
	if r.Method != "GET" {
		http.Error(w, "Only GET is supported for /list", 400)
		return
	}

	marshalled, err := json.Marshal(days)
	if err != nil {
		fmt.Printf("Got an error while converting challenges to JSON: %v", err)
		os.Exit(1)
	}
	w.Header().Set("Content-Type", "application/json")
	fmt.Fprint(w, string(marshalled))
}

type SolveRequest struct {
	ID    int    `json:"id"`
	Input string `json:"input"`
}

func solve(w http.ResponseWriter, r *http.Request) {
	if r.Method != "POST" {
		http.Error(w, "Only POST is supported for /solve", 400)
		return
	}

	dec := json.NewDecoder(r.Body)
	var data SolveRequest
	if err := dec.Decode(&data); err != nil {
		http.Error(w, fmt.Sprintf("Got an error while parsing the body: %v", err), 400)
		return
	}

	var day *Day
	for _, d := range days {
		if d.ID == data.ID {
			day = &d
		}
	}
	if day == nil {
		http.Error(w, fmt.Sprintf("ID %d did not match a day", data.ID), 400)
		return
	}

	solved, err := day.solveFunc(data.Input)
	if err != nil {
		http.Error(w, fmt.Sprintf("Got an error while solving: %v", err), 400)
		return
	}
	fmt.Fprint(w, solved)
}

func logRequest(handler http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		log.Printf("%s %s %s\n", r.RemoteAddr, r.Method, r.URL)
		handler.ServeHTTP(w, r)
	})
}

func main() {
	http.HandleFunc("/list/", list)
	http.HandleFunc("/solve/", solve)
	port := os.Getenv("CHALLENGES_AOC_2019_PORT")
	if port == "" {
		port = "8082"
	}
	log.Printf("Listening on port %s", port)
	log.Fatal(http.ListenAndServe(":"+port, logRequest(http.DefaultServeMux)))
}
