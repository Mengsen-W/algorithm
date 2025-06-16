// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimizeMax(nums []int, p int) int {
	sort.Ints(nums)

	check := func(mx int) bool {
		cnt := 0
		for i := 0; i < len(nums)-1; i++ {
			if nums[i+1]-nums[i] <= mx {
				cnt++
				i++
			}
		}
		return cnt >= p
	}

	left, right := 0, nums[len(nums)-1]-nums[0]
	for left < right {
		mid := (left + right) / 2
		if check(mid) {
			right = mid
		} else {
			left = mid + 1
		}
	}
	return left
}

func main() {
	tests := []struct {
		nums []int
		p    int
		ans  int
	}{
		{[]int{10, 1, 2, 7, 1, 3}, 2, 1},
		{[]int{4, 2, 1, 2}, 1, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimizeMax(test.nums, test.p), index)
	}
}
