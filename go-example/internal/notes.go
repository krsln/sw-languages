package main

import (
	"fmt"
	"sync"
)

func main() {

	// arrays & slices in Go
	var list = [3]string{"Apple", "Orange", "Banana"} // fixed size
	var slice = []string{"Apple", "Orange", "Banana"} // dynamic size

	fmt.Println(list, slice)
	fmt.Println("-----------------------------------")

	var username string
	fmt.Print("Enter your username: ")
	scan, _ := fmt.Scan(&username)
	fmt.Println("Hello, "+username, scan)

	// goroutine -----------------------------
	var wg sync.WaitGroup

	// Set the counter to 2, one for each goroutine
	wg.Add(2)

	// "go" keyword
	go someFunction(&wg)
	go func() {
		defer wg.Done() // Call Done() when this goroutine finishes
		fmt.Println("Hello from a goroutine")
	}()

	// Wait for all goroutines to finish
	wg.Wait()
	// ---------------------------------------

	// _ for ignoring variables
	// Capitalize method name make it public

	fmt.Println("Exiting main.")
}

func someFunction(wg *sync.WaitGroup) {
	defer wg.Done() // Call Done() when this goroutine finishes
	fmt.Println("goroutine | someFunction")
}
