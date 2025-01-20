// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findClosestNumber(nums []int) int {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	res := nums[0]      // 已遍历元素中绝对值最小且数值最大的元素
	dis := abs(nums[0]) // 已遍历元素的最小绝对值
	for _, num := range nums {
		if abs(num) < dis {
			dis = abs(num)
			res = num
		} else if abs(num) == dis {
			res = max(res, num)
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{-4, -2, 1, 4, 8}, 1},
		{[]int{2, -1, 1}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findClosestNumber(test.nums), index)
	}
}
