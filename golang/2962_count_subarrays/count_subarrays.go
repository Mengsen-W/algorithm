// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countSubarrays(nums []int, k int) int64 {
	mx := nums[0]
	for _, v := range nums {
		if v > mx {
			mx = v
		}
	}
	var ans int64 = 0
	cnt := 0
	left := 0
	for _, x := range nums {
		if x == mx {
			cnt++
		}
		for cnt == k {
			if nums[left] == mx {
				cnt--
			}
			left++
		}
		ans += int64(left)
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int64
	}{
		{[]int{1, 3, 2, 3, 3}, 2, 6},
		{[]int{1, 4, 2, 1}, 3, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countSubarrays(test.nums, test.k), index)
	}
}
