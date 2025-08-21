package main

import "fmt"

// SLICES
func main() {
	fmt.Println("---------------------------------------", "Slices")
	// declaration of slice of integers
	{
		//var numbers []int
		//numbers = make([]int, 5)
		numbers := make([]int, 5)
		// initialization of slice of integers with length 5 filled with [0,0,0,0,0]
		fmt.Println("Slice is : ", numbers)
	}

	fmt.Println("---------------------------------------", "Sub-slicing")
	// Sub-slicing
	numbers2 := []int{0, 1, 2, 3, 4, 5, 6, 7, 8}
	subSlice1 := numbers2[1:4]
	subSlice2 := numbers2[:3]
	subSlice3 := numbers2[4:]

	fmt.Println("subSlice1:", subSlice1)
	fmt.Println("subSlice2:", subSlice2)
	fmt.Println("subSlice3:", subSlice3)

	fmt.Println("---------------------------------------", "Append() and Copy()")
	// Append() and Copy()

	numbers := []int{}
	// Appending elements
	numbers = append(numbers, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10)

	numbers1 := make([]int, len(numbers), cap(numbers)*2) // 'cap*2' need an explanation ??
	// Copying contents
	copy(numbers1, numbers)

	fmt.Println("numbers slice:", numbers, cap(numbers))
	fmt.Println("numbers1 slice after copy:", numbers1, cap(numbers1))

	fmt.Println("---------------------------------------", "Nil Slice")
	// Nil Slice
	var num []int // nil slice
	fmt.Println("Number:", num)
	if num == nil {
		fmt.Println("Slice is nil")
	}
}
