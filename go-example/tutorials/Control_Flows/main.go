package main

import "fmt"

/*
	Control Flow

Conditionals (If Else statement)
Loops
*/
func main() {
	fmt.Println("---------------------------------------", "If Statement")
	// If Statement
	// Executes a block of code if the condition is true.
	age := 18
	if age >= 18 {
		fmt.Println("You are eligible to vote")
	} else {
		fmt.Println("You are not eligible to vote")
	}

	fmt.Println("---------------------------------------", "Nested Statement")
	// Nested Statements
	score := 95
	if score >= 90 {
		fmt.Println("Grade:A")
	} else if score >= 80 {
		if score >= 85 {
			fmt.Println("Grade:B+")
		} else {
			fmt.Println("Grade:B")
		}
	} else {
		fmt.Println("Grade:C")
	}

	fmt.Println("---------------------------------------", "Loops")
	//Control Flow: Loops

	// 1* For Loop
	for i := 0; i < 10; i++ {
		fmt.Println(i)
	}

	// 2* Nested Loop Control
	// Multiplication Table
	for i := 1; i <= 5; i++ {
		for j := 1; j <= 5; j++ {
			fmt.Printf("%d * %d = %d\t", i, j, i*j)
		}
		fmt.Println()
	}

	fmt.Println("---------------------------------------", "Loop control statements")
	// Loop control statements: like Break, Continue and GoTo
	//A Break
	for i := 0; i < 10; i++ {
		if i == 5 {
			break
		}
		fmt.Println(i)
	}
	//B Continue
	for i := 0; i < 10; i++ {
		if i%2 == 0 {
			continue
		}
		fmt.Println(i)
	}
	//C GoTo
	{
		for i := 0; i < 10; i++ {
			fmt.Println(i)
			if i == 5 {
				goto end
			}
		}
	end:
		fmt.Println("Loop ended")
	}

	//Second example of Goto
	{
		i := 0
	start:
		if i < 10 {
			fmt.Println(i)
			i++
			goto start
		}
	}
	//Infinite Loop
	counter := 0
	for {
		fmt.Println("Infinite Loop")
		counter++
		if counter > 5 {
			break
		}
	}
	//Basic way ?
	{
	start_:
		fmt.Println("This is an infinite loop!")
		counter++
		if counter > 10 {
			return
		}
		goto start_
	}
}

// Basic
// 10 PRINT "This is an infinite loop!"
// 20 GOTO 10
