// Package main ...
package main

import (
	"sort"
	"strconv"
)

func maxAlternatingSum(nums []int) int64 {
	n := len(nums)
	for i := range nums {
		nums[i] *= nums[i]
	}
	sort.Ints(nums)

	var ans int64
	for i := 0; i < n/2; i++ {
		ans -= int64(nums[i])
	}
	for i := n / 2; i < n; i++ {
		ans += int64(nums[i])
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{1, 2, 3}, 12},
		{[]int{1, -1, 2, -2, 3, -3}, 16},
	}

	for _, test := range tests {
		if got := maxAlternatingSum(test.nums); got != test.ans {
			panic("got " + strconv.Itoa(int(got)) + " want " + strconv.Itoa(int(test.ans)))
		}
	}
}
