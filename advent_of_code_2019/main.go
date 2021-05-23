package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"strings"
	"time"
)

type Day struct {
	ID          string `json:"id"`
	Title       string `json:"title"`
	Description string `json:"description"`
	solveFunc   func(string) (string, error)
}

var days []Day

func registerDay(title, description string, solveFunc func(string) (string, error)) {
	days = append(days, Day{ID: fmt.Sprintf("%d", len(days)+1), Title: title, Description: description, solveFunc: solveFunc})
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

func solve(w http.ResponseWriter, r *http.Request) {
	if r.Method != "POST" {
		http.Error(w, "Only POST is supported for /solve", 400)
		return
	}

	id := strings.Trim(strings.TrimPrefix(r.URL.Path, "/solve/"), "/")

	var day *Day
	for _, d := range days {
		if d.ID == id {
			day = &d
			break
		}
	}
	if day == nil {
		http.Error(w, fmt.Sprintf("ID %s did not match a day", id), 400)
		return
	}

	bytes, err := ioutil.ReadAll(r.Body)
	if err != nil {
		http.Error(w, fmt.Sprintf("Error reading the request's body: %v", err), 400)
		return
	}

	solved, err := day.solveFunc(string(bytes))
	if err != nil {
		http.Error(w, fmt.Sprintf("Got an error while solving: %v", err), 400)
		return
	}
	fmt.Fprint(w, solved)
}

func logRequest(handler http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		log.Printf("%s %s\n", r.Method, r.URL)
		handler.ServeHTTP(w, r)
	})
}

type logWriter struct {
}

func (writer logWriter) Write(bytes []byte) (int, error) {
	return fmt.Fprint(os.Stderr, time.Now().UTC().Format("2006-01-02T15:04:05.999Z")+" [AoC 2019] "+string(bytes))
}

func main() {
	log.SetFlags(0)
	log.SetOutput(new(logWriter))

	http.HandleFunc("/list/", list)
	http.HandleFunc("/solve/", solve)
	port := os.Getenv("CHALLENGES_AOC_2019_PORT")
	if port == "" {
		port = "8082"
	}
	log.Printf("Listening on port %s", port)
	log.Fatal(http.ListenAndServe(":"+port, logRequest(http.DefaultServeMux)))
}
