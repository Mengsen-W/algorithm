// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func medianOfUniquenessArray(nums []int) int {
	n := len(nums)
	median := (int64(n)*int64(n+1)/2 + 1) / 2

	// 检测数组中不同元素数目小于等于 t 的连续子数组数目是否大于等于 median
	check := func(t int) bool {
		cnt := make(map[int]int)
		tot := int64(0)
		for i, j := 0, 0; i < n; i++ {
			cnt[nums[i]]++
			for len(cnt) > t {
				cnt[nums[j]]--
				if cnt[nums[j]] == 0 {
					delete(cnt, nums[j])
				}
				j++
			}
			tot += int64(i - j + 1)
		}
		return tot >= median
	}

	res := 0
	lo, hi := 1, n
	for lo <= hi {
		mid := (lo + hi) / 2
		if check(mid) {
			res = mid
			hi = mid - 1
		} else {
			lo = mid + 1
		}
	}

	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3}, 1},
		{[]int{3, 4, 3, 4, 5}, 2},
		{[]int{4, 3, 5, 4}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, medianOfUniquenessArray(test.nums), index)
	}
}
