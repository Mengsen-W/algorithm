// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxFrequencyElements(nums []int) int {
	count := make(map[int]int)
	for _, a := range nums {
		count[a]++
	}
	maxf := 0
	for _, freq := range count {
		if freq > maxf {
			maxf = freq
		}
	}
	res := 0
	for _, freq := range count {
		if freq == maxf {
			res += maxf
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 2, 3, 1, 4}, 4},
		{[]int{1, 2, 3, 4, 5}, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxFrequencyElements(test.nums), index)
	}
}
