// Package main ...
package main

import (
	"fmt"
	"reflect"
)

func leftRightDifference(nums []int) []int {
	n := len(nums)
	ans := make([]int, n)

	leftSum := 0
	for i := 0; i < n; i++ {
		ans[i] = leftSum
		leftSum += nums[i]
	}

	rightSum := 0
	for i := n - 1; i >= 0; i-- {
		ans[i] = abs(ans[i] - rightSum)
		rightSum += nums[i]
	}

	return ans
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{10, 4, 8, 3}, []int{15, 1, 11, 22}},
		{[]int{1}, []int{0}},
	}

	for _, test := range tests {
		result := leftRightDifference(test.nums)
		if !reflect.DeepEqual(result, test.ans) {
			fmt.Printf("leftRightDifference(%v) = %v, want %v\n", test.nums, result, test.ans)
		}
	}
}
