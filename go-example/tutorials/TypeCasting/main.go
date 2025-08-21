package main

import (
	"fmt"
	"strconv"
)

// Type Casting
func main() {
	fmt.Println("Type Casting")

	var total int = 20
	var items int = 7
	var average float64

	average = float64(total) / float64(items)
	fmt.Printf("Average: %.2f\n", average)

	// Converting Strings to integers
	str := "5254"
	num, err := strconv.Atoi(str)
	if err != nil {
		fmt.Println("error converting: ", err)
		return
	}
	fmt.Printf("converted number: %v\n", num)
}
