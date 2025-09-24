// Package main ...
package main

import (
	"math"
	"slices"
	"testing"

	"github.com/stretchr/testify/assert"
)

func partitionArray(nums []int, k int) (ans int) {
	slices.Sort(nums)
	mn := math.MinInt / 2 // 防止减法溢出
	for _, x := range nums {
		if x-mn > k { // 必须分割
			ans++
			mn = x // mn 是下一段的最小值
		}
	}
	return
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{3, 6, 1, 2, 5}, 2, 2},
		{[]int{1, 2, 3}, 1, 2},
		{[]int{2, 2, 4, 5}, 0, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, partitionArray(test.nums, test.k), index)
	}
}
