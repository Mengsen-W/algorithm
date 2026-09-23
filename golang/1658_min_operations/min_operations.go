// Package main ...
package main

import "fmt"

func minOperations(nums []int, x int) int {
	min := func(a, b int) int {
		if b < a {
			return b
		}
		return a
	}
	n := len(nums)
	sum := 0
	for _, num := range nums {
		sum += num
	}
	if sum < x {
		return -1
	}

	right := 0
	lsum := 0
	rsum := sum
	ans := n + 1

	for left := -1; left < n; left++ {
		if left != -1 {
			lsum += nums[left]
		}
		for right < n && lsum+rsum > x {
			rsum -= nums[right]
			right++
		}
		if lsum+rsum == x {
			ans = min(ans, (left+1)+(n-right))
		}
	}
	if ans > n {
		return -1
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		x    int
		ans  int
	}{
		{[]int{1, 1, 4, 2, 3}, 5, 2},
		{[]int{5, 6, 7, 8, 9}, 4, -1},
		{[]int{3, 2, 20, 1, 1, 3}, 10, 5},
	}

	for index, test := range tests {
		if test.ans != minOperations(test.nums, test.x) {
			fmt.Println("Test", index+1, "failed")
		}
	}
}
