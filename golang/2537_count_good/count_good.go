// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countGood(nums []int, k int) int64 {
	n := len(nums)
	same, right := 0, -1
	cnt := make(map[int]int)
	var ans int64 = 0
	for left := 0; left < n; left++ {
		for same < k && right+1 < n {
			right++
			same += cnt[nums[right]]
			cnt[nums[right]]++
		}
		if same >= k {
			ans += int64(n - right)
		}
		cnt[nums[left]]--
		same -= cnt[nums[left]]
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int64
	}{
		{[]int{1, 1, 1, 1, 1}, 10, 1},
		{[]int{3, 1, 4, 3, 2, 2, 4}, 2, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countGood(test.nums, test.k), index)
	}
}
