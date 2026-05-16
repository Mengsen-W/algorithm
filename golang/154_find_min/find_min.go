// Package main ...
package main

import "fmt"

func findMin(nums []int) int {
	low, high := 0, len(nums)-1
	for low < high {
		pivot := low + (high-low)/2
		if nums[pivot] < nums[high] {
			high = pivot
		} else if nums[pivot] > nums[high] {
			low = pivot + 1
		} else {
			high--
		}
	}
	return nums[low]
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 3, 5}, 1},
		{[]int{2, 2, 2, 0, 1}, 0},
	}
	for _, t := range tests {
		if got := findMin(t.nums); got != t.ans {
			panic(fmt.Sprintf("findMin(%v) = %v; want %v", t.nums, got, t.ans))
		}
	}
}
