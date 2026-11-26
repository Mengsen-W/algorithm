// Package main ...
package main

import (
	"math"
)

func canAliceWin(n int) bool {
	k := int((21 - math.Sqrt(float64(441-8*n))) / 2)
	return k%2 == 1
}

func main() {
	tests := []struct {
		n   int
		ans bool
	}{
		{12, true},
		{1, false},
	}

	for index, test := range tests {
		if got := canAliceWin(test.n); got != test.ans {
			panic(index)
		}
	}
}
