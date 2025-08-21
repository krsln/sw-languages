package main

import "fmt"

/*
Understanding Variables Declarations in Go
*/
func main() {
	fmt.Println("---------------------------------------")
	// way1: Declare and assign on 2 different lines
	var mango string
	mango = "This is a big mango!"
	var weight int
	weight = 54

	// way2: Declare and assign on the same line
	var height int = 23

	fmt.Println("Mango:", mango)
	fmt.Println("weight:", weight)
	fmt.Println("height:", height)

	fmt.Println("---------------------------------------")
	// way3 (shorthand)
	// shorthand notation :=
	// var age = 23 // Type inference
	age := 54
	city := "Washington"
	fmt.Println("My age is:", age)
	fmt.Println("My city is:", city)

	// Multiple declaration and init. on same line
	var apples, oranges int = 23, 78
	fmt.Println("I have", apples, "apples and", oranges, "oranges")

	var fruits = apples + oranges
	fmt.Println("fruits:", fruits)

	var windows, mac, linux string = "\nWindows is ok\n", "Mac is meh\n", "Linux is the GOAT!\n"
	fmt.Print(windows, mac, linux)

	fmt.Println("---------------------------------------")
	// Static Type declaration
	var x float64 = 20.5
	fmt.Printf("var x float64 = %v \tx is of type: %T\n", x, x)

	// Dynamic Type declaration
	y := 89
	fmt.Printf("y := %v \ty is of type: %T\n", y, y)

	fmt.Println("---------------------------------------")
	// Mixed Variable Declaration
	var a, b, c = 758.52, 8, "foobar"
	fmt.Println(a)
	fmt.Println(b)
	fmt.Println(c)
	fmt.Printf("a is of type: %T\n", a)
	fmt.Printf("b is of type: %T\n", b)
	fmt.Printf("c is of type: %T\n", c)
}
