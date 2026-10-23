// Package main ...
package main

import (
	"math"
	"strconv"
)

func sortPermutation(nums []int) int {
	n := len(nums)
	res := math.MaxInt32
	for i := 0; i < n; i++ {
		if nums[i] == i {
			continue
		}
		res &= nums[i]
	}
	if res == math.MaxInt32 {
		return 0
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{0, 3, 2, 1}, 1},
		{[]int{0, 1, 3, 2}, 2},
		{[]int{3, 2, 1, 0}, 0},
	}

	for _, test := range tests {
		if got := sortPermutation(test.nums); got != test.ans {
			panic("got " + strconv.Itoa(got) + " want " + strconv.Itoa(test.ans))
		}
	}
}
