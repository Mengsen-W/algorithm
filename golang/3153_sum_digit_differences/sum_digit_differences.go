// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func sumDigitDifferences(nums []int) int64 {
	res := int64(0)
	for nums[0] > 0 {
		cnt := make([]int, 10)
		for i := 0; i < len(nums); i++ {
			cnt[nums[i]%10]++
			nums[i] /= 10
		}
		for i := 0; i < 10; i++ {
			res += int64(len(nums)-cnt[i]) * int64(cnt[i])
		}
	}
	return res / 2
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{13, 23, 12}, 4},
		{[]int{10, 10, 10, 10}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, sumDigitDifferences(test.nums), index)
	}
}
