// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countCompleteSubarrays(nums []int) int {
	res := 0
	cnt := make(map[int]int)
	n := len(nums)
	right := 0
	distinct := make(map[int]bool)
	for _, num := range nums {
		distinct[num] = true
	}
	distinctCount := len(distinct)
	for left := 0; left < n; left++ {
		if left > 0 {
			remove := nums[left-1]
			cnt[remove]--
			if cnt[remove] == 0 {
				delete(cnt, remove)
			}
		}
		for right < n && len(cnt) < distinctCount {
			add := nums[right]
			cnt[add]++
			right++
		}
		if len(cnt) == distinctCount {
			res += (n - right + 1)
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 3, 1, 2, 2}, 4},
		{[]int{5, 5, 5, 5}, 10},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countCompleteSubarrays(test.nums), index)
	}
}
