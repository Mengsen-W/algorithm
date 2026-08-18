// Package main ...
package main

import "fmt"

func largestInteger(nums []int, k int) int {
	n := len(nums)
	if n == k {
		res := nums[0]
		for _, x := range nums {
			if x > res {
				res = x
			}
		}
		return res
	}
	count := make([]int, 51)
	for _, x := range nums {
		count[x]++
	}
	if k == 1 {
		for i := 50; i >= 0; i-- {
			if count[i] == 1 {
				return i
			}
		}
		return -1
	}
	res := -1
	if count[nums[0]] == 1 {
		res = max(res, nums[0])
	}
	if count[nums[n-1]] == 1 {
		res = max(res, nums[n-1])
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{3, 9, 2, 1, 7}, 3, 7},
		{[]int{3, 9, 7, 2, 1, 7}, 4, 3},
		{[]int{0, 0}, 1, -1},
	}

	for _, test := range tests {
		if got := largestInteger(test.nums, test.k); got != test.ans {
			panic(fmt.Sprintf("test=%+v, got=%v, want=%v", test, got, test.ans))
		}
	}
}
