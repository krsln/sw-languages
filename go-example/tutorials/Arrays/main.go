package main

import "fmt"

// Declaring Array
var balance1 [10]float32

// Initializing Array at declaration
var balance2 = [5]float32{12.20, 23.34, 45.34, 65.23, 10.20}

// Initialize while omitting the size of the array elements (slice)
var balance3 = []float32{12.20, 23.34, 45.34, 65.23, 10.20}

/*
	Understanding Arrays in Golang

[ int ]
[ string ]
[int, string] ❌ (not allowed, arrays are homogenous)
*/
func main() {
	/*
		Fixed Size: The size of an array is fixed at declaration.
		Type-Specific: All elements must be of the same type.
		Contiguous Memory: Provides fast access to elements.
		Zero Indexing: The first element is accessed with index 0.
	*/

	var n [10]int
	// Initialize array elements
	for i := 0; i < 10; i++ {
		n[i] = i + 100
	}

	// Display array elements
	for i := 0; i < 10; i++ {
		fmt.Printf("Element[%d] = %d\n", i, n[i])
	}

	// Example of using balance arrays
	fmt.Println("balance_1:", balance1)
	fmt.Println("balance_2:", balance2)
	fmt.Println("balance_3:", balance3)
}
