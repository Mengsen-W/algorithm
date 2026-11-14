// Package main ...
package main

import (
	"math"
	"strconv"
)

func minimumSumSubarray(nums []int, l int, r int) int {
	n := len(nums)
	minSum := math.MaxInt

	initial := 0
	for i := 0; i < l-1; i++ {
		initial += nums[i]
	}

	for length := l; length <= r; length++ {
		initial += nums[length-1]
		sum := initial

		if sum > 0 && sum < minSum {
			minSum = sum
		}

		for hi := length; hi < n; hi++ {
			sum += nums[hi] - nums[hi-length]
			if sum > 0 && sum < minSum {
				minSum = sum
			}
		}
	}

	if minSum == math.MaxInt {
		return -1
	}
	return minSum
}

func main() {
	tests := []struct {
		nums []int
		l    int
		r    int
		ans  int
	}{
		{[]int{3, -2, 1, 4}, 2, 3, 1},
		{[]int{-2, 2, -3, 1}, 2, 3, -1},
		{[]int{1, 2, 3, 4}, 2, 4, 3},
	}

	for _, test := range tests {
		if got := minimumSumSubarray(test.nums, test.l, test.r); got != test.ans {
			panic("got " + strconv.Itoa(got) + " want " + strconv.Itoa(test.ans))
		}
	}
}
