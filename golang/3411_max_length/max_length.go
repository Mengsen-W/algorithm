// Package main ...
package main

import "strconv"

func maxLength(nums []int) int {
	res := 2
	prodVal := 1
	n := len(nums)
	start, end := 0, 0

	gcd := func(a, b int) int {
		for b != 0 {
			a, b = b, a%b
		}
		return a
	}

	for end < n {
		for gcd(prodVal, nums[end]) > 1 {
			prodVal /= nums[start]
			start++
		}
		prodVal *= nums[end]
		if end-start+1 > res {
			res = end - start + 1
		}
		end++
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 1, 2, 1, 1, 1}, 5},
		{[]int{2, 3, 4, 5, 6}, 3},
		{[]int{1, 2, 3, 1, 4, 5, 1}, 5},
	}

	for _, test := range tests {
		if got := maxLength(test.nums); got != test.ans {
			panic("got " + strconv.Itoa(got) + " expected " + strconv.Itoa(test.ans))
		}
	}
}
