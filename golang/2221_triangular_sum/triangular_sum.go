// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func triangularSum(nums []int) int {
	current := nums
	for len(current) > 1 {
		newNums := make([]int, 0, len(current)-1)
		for i := 0; i < len(current)-1; i++ {
			newNums = append(newNums, (current[i]+current[i+1])%10)
		}
		current = newNums
	}
	return current[0]
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3, 4, 5}, 8},
		{[]int{5}, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, triangularSum(test.nums), "test %d", index)
	}
}
