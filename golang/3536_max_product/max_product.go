// Package main ...
package main

import "fmt"

func maxProduct(n int) int {
	first, second := 0, 0
	for n > 0 {
		x := n % 10
		if x > first {
			second = first
			first = x
		} else if x > second {
			second = x
		}
		n /= 10
	}
	return first * second
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{31, 3},
		{22, 4},
		{124, 8},
	}

	for _, test := range tests {
		if maxProduct(test.n) != test.ans {
			fmt.Println("test failed")
		}
	}
}
