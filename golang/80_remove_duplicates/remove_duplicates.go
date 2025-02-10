// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func removeDuplicates(nums []int) int {
	n := len(nums)
	if n <= 2 {
		return n
	}
	slow, fast := 2, 2
	for fast < n {
		if nums[slow-2] != nums[fast] {
			nums[slow] = nums[fast]
			slow++
		}
		fast++
	}
	return slow
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 1, 1, 2, 2, 3}, 5},
		{[]int{0, 0, 1, 1, 1, 1, 2, 3, 3}, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, removeDuplicates(test.nums), index)
	}
}
