// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countInterestingSubarrays(nums []int, modulo int, k int) int64 {
	n := len(nums)
	cnt := make(map[int]int)
	var res int64 = 0
	var prefix int = 0
	cnt[0] = 1
	for i := 0; i < n; i++ {
		if nums[i]%modulo == k {
			prefix++
		}
		res += int64(cnt[(prefix-k+modulo)%modulo])
		cnt[prefix%modulo]++
	}
	return res
}

func main() {
	tests := []struct {
		nums   []int
		modulo int
		k      int
		ans    int64
	}{
		{[]int{3, 2, 4}, 2, 1, 3},
		{[]int{3, 1, 9, 6}, 3, 0, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countInterestingSubarrays(test.nums, test.modulo, test.k), index)
	}
}
