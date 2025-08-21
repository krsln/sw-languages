package main

import (
	"fmt"
	"math"
)

// Shape Define an interface
type Shape interface {
	area() float64
}

// Circle Define a Circle struct
type Circle struct {
	radius float64
}

// Rectangle Define a Rectangle struct
type Rectangle struct {
	width, height float64
}

// Implement area method for Circle
func (c Circle) area() float64 {
	return math.Pi * c.radius * c.radius
}

// Implement area method for Rectangle
func (r Rectangle) area() float64 {
	return r.width * r.height
}

// Function to get area of any shape!
// Polymorphism in Action
func getArea(s Shape) float64 {
	return s.area()
}

// Interfaces
func main() {
	fmt.Println("Interfaces")

	/*
		// Define an interface
		type myInterface interface {
			// methods
			// MethodName1 ReturnType
			// MethodName2 ReturnType
			// MethodName3 ReturnType
		}

		//Define a struct
		type myStruct struct {
			// variables
		}
	*/

	circle := Circle{radius: 5}
	rectangle := Rectangle{width: 10, height: 5}

	fmt.Printf("Circle area: %f\n", getArea(circle))
	fmt.Printf("Rectangle area: %f\n", getArea(rectangle))
}
