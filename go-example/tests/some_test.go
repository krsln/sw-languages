package tests

import (
	"testing"
)

// Add sums two integers and returns the result.
func Add(x, y int) int {
	return x + y
}

func TestAdd(t *testing.T) {
	sum := Add(1, 2)
	expected := 3
	if sum != expected {
		t.Errorf("Expected %d, got %d", expected, sum)
	}
}
