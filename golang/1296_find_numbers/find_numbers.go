// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func findNumbers(nums []int) int {
	ans := 0
	for _, num := range nums {
		if int(math.Log10(float64(num))+1)%2 == 0 {
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{12, 345, 2, 6, 7896}, 2},
		{[]int{555, 901, 482, 1771}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findNumbers(test.nums), index)
	}
}
