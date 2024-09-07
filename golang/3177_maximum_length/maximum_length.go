// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumLength(nums []int, k int) int {
	lenNums := len(nums)
	dp := make(map[int][]int)
	zd := make([]int, k+1)

	for i := 0; i < lenNums; i++ {
		v := nums[i]
		if _, ok := dp[v]; !ok {
			dp[v] = make([]int, k+1)
		}

		tmp := dp[v]
		for j := 0; j <= k; j++ {
			tmp[j]++
			if j > 0 {
				tmp[j] = max(tmp[j], zd[j-1]+1)
			}
		}

		for j := 0; j <= k; j++ {
			zd[j] = max(zd[j], tmp[j])
		}
	}
	return zd[k]
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{1, 2, 1, 1, 3}, 2, 4},
		{[]int{1, 2, 3, 4, 5, 1}, 0, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumLength(test.nums, test.k), index)
	}
}
