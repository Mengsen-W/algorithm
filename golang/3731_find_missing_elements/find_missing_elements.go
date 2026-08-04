// Package main ...
package main

import (
	"fmt"
	"slices"
)

func findMissingElements(nums []int) []int {
	slices.Sort(nums)
	ans := []int{}
	for i := 0; i < len(nums)-1; i++ {
		for j := nums[i] + 1; j < nums[i+1]; j++ {
			ans = append(ans, j)
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{1, 4, 2, 5}, []int{3}},
		{[]int{7, 8, 6, 9}, []int{}},
		{[]int{5, 1}, []int{2, 3, 4}},
	}

	for _, test := range tests {
		result := findMissingElements(test.nums)
		if (result == nil && test.ans == nil) || slices.Equal(result, test.ans) {
			fmt.Println("test passed")
		} else {
			fmt.Println("test failed", result, test.ans)
		}
	}
}
