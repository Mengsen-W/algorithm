// Package main ...
package main

import "fmt"

func sumAndMultiply(n int) int64 {
	var x int64 = 0
	var sum int64 = 0
	var pow10 int64 = 1
	for n > 0 {
		d := n % 10
		sum += int64(d)
		if d > 0 {
			x += int64(d) * pow10
			pow10 *= 10
		}
		n /= 10
	}
	return x * sum
}

func main() {
	tests := []struct {
		n   int
		ans int64
	}{
		{10203004, 12340},
		{1000, 1},
	}

	for index, test := range tests {
		if sumAndMultiply(test.n) != test.ans {
			fmt.Printf("test %d failed: expected %d\n", index, test.ans)
		}
	}
}
