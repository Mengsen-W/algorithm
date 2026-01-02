// Package main ...
package main

import (
	"math/rand"
	"testing"

	"github.com/stretchr/testify/assert"
)

func repeatedNTimes(nums []int) int {
	n := len(nums)
	for {
		x, y := rand.Intn(n), rand.Intn(n)
		if x != y && nums[x] == nums[y] {
			return nums[x]
		}
	}
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3}, 3},
		{[]int{2, 1, 2, 5, 3, 2}, 2},
		{[]int{5, 1, 5, 2, 5, 3, 5, 4}, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, repeatedNTimes(test.nums), test.ans, index)
	}
}
