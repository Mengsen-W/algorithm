// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countSymmetricIntegers(low int, high int) int {
	res := 0
	for a := low; a <= high; a++ {
		if a < 100 && a%11 == 0 {
			res++
		} else if 1000 <= a && a < 10000 {
			left := a/1000 + (a%1000)/100
			right := (a%100)/10 + a%10
			if left == right {
				res++
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		low  int
		high int
		ans  int
	}{
		{1, 100, 9},
		{1200, 1230, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countSymmetricIntegers(test.low, test.high), index)
	}
}
