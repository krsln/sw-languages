package main

import "fmt"

/*
	Constants in go programming language

Constants are immutable values known at compile time and cannot be changed
*/
func main() {
	fmt.Println("---------------------------------------")
	// Declare strings and numeric constants
	const NAME string = "John Doe"
	const PRICE int = 100

	fmt.Println("Name is : ", NAME)
	fmt.Println("Price is :", PRICE)

	fmt.Println("---------------------------------------")
	// Declare the integer literals
	// An integer literal can be decimal, octal, or hexadecimal.
	// 0x - hex, 0 for octal
	const (
		DECIMAL     = 255  //decimals with no prefix
		OCTAL       = 0377 // octal with leading 0
		HEXADECIMAL = 0xff // hexadecimal with leading 0x
	)
	fmt.Println("Decimal:", DECIMAL, "Octal:", OCTAL, "Hexadecimal:", HEXADECIMAL)

	fmt.Println("---------------------------------------")
	// Floating-point Literals
	// A floating-point literal can have an integer part, a decimal point, a fractional part, and an exponent part.
	const PI float64 = 3.141
	fmt.Println("PI value is : ", PI)
	const AVOGADRO = 6.022e23
	fmt.Println("AVOGADRO value is : ", AVOGADRO)

	fmt.Println("---------------------------------------")
	// Escape Sequences in string literals
	// this is a new line
	const GREETING = "Hello, Earth!"
	// this is a quote
	const QUOTE = "\"GO is simple!\" - A programmer!"
	fmt.Println(GREETING)
	fmt.Println(QUOTE)
	// Alert or Bell \a
	const BELL = "Bell is \a"
	fmt.Println(BELL)
	// Line Breaks
	const LB = "I\nAM\nBATMAN\n!"
	fmt.Println(LB)

	fmt.Println("---------------------------------------")
	// Multi-line and Concatenated String Literals
	const MULTILINE = "The Eiffel tower is so long that it needs to" +
		"span multiple clouds for better\nphotographing in France!"
	const CONCATENATED = "Concatenated " + "string"
	fmt.Println(MULTILINE)
	fmt.Println(CONCATENATED)

	fmt.Println("---------------------------------------")
	// Boolean Constants
	const ACTIVE = true
	const READY = false
	fmt.Println("ACTIVE:", ACTIVE, " READY:", READY)

	fmt.Println("---------------------------------------")
	// Constants for calculations
	const LENGTH = 50
	const WIDTH = 5
	const AREA = LENGTH * WIDTH
	fmt.Println("AREA:", AREA)
}
