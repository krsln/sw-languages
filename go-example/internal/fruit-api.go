package main

import (
	"fmt"
	"log"
	"net/http"
)

// FruitList is a global slice to store the list of fruits
var FruitList = []string{"Apple", "Orange", "Banana"}

func main() {
	// Define HTTP handlers
	http.HandleFunc("/fruits", listFruitsHandler)
	http.HandleFunc("/add", addFruitHandler)

	// Print startup message
	fmt.Println("Starting server at http://localhost:8080")
	fmt.Println("Visit http://localhost:8080/fruits to see the fruit list")
	fmt.Println("Visit http://localhost:8080/add?fruit=<name> to add a fruit (e.g., /add?fruit=Grape)")

	// Start the server
	err := http.ListenAndServe(":8080", nil)
	if err != nil {
		log.Fatalf("Error starting server: %v", err)
	}
}

// listFruitsHandler displays the current list of fruits
func listFruitsHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Fruit List:\n")
	printAll(w, FruitList)
}

// addFruitHandler adds a new fruit to the FruitList
func addFruitHandler(w http.ResponseWriter, r *http.Request) {
	// Get the 'fruit' query parameter from the URL
	newFruit := r.URL.Query().Get("fruit")
	if newFruit == "" {
		http.Error(w, "Missing 'fruit' query parameter", http.StatusBadRequest)
		return
	}

	// Add the new fruit to the global FruitList
	FruitList = addItem(FruitList, newFruit)

	// Respond with the updated list
	fmt.Fprintf(w, "Added %s successfully!\nUpdated Fruit List:\n", newFruit)
	printAll(w, FruitList)
}

// printAll writes each item in the slice to the response writer with a numbered prefix
func printAll(w http.ResponseWriter, items []string) {
	for index, item := range items {
		fmt.Fprintf(w, "%d. %s\n", index+1, item)
	}
}

// addItem appends a new item to the slice and returns the updated slice
func addItem(items []string, newItem string) []string {
	if newItem == "" {
		log.Println("Warning: Attempted to add an empty item")
		return items
	}
	return append(items, newItem)
}
