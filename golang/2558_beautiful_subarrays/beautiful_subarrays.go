// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func beautifulSubarrays(nums []int) int64 {
	cnt := make(map[int]int)
	mask := 0
	var ans int64 = 0
	cnt[0] = 1
	for _, x := range nums {
		mask ^= x
		ans += int64(cnt[mask])
		cnt[mask]++
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{4, 3, 1, 2, 4}, 2},
		{[]int{1, 10, 4}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, beautifulSubarrays(test.nums), index)
	}
}
