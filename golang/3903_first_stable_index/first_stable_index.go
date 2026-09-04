// Package main ...
package main

import "fmt"

func firstStableIndex(nums []int, k int) int {
	n := len(nums)
	for i := 0; i < n; i++ {
		maxValue := nums[i]
		minValue := nums[i]
		for j := 0; j < i; j++ {
			if nums[j] > maxValue {
				maxValue = nums[j]
			}
		}
		for j := i + 1; j < n; j++ {
			if nums[j] < minValue {
				minValue = nums[j]
			}
		}
		if maxValue-minValue <= k {
			return i
		}
	}
	return -1
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{5, 0, 1, 4}, 3, 3},
		{[]int{3, 2, 1}, 1, -1},
		{[]int{0}, 0, 0},
	}

	for index, test := range tests {
		result := firstStableIndex(test.nums, test.k)
		if result != test.ans {
			fmt.Printf("Test %d failed: expected %d, got %d\n", index, test.ans, result)
		} else {
			fmt.Printf("Test %d passed\n", index)
		}
	}
}

