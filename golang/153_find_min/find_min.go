// Package main ...
package main

func findMin(nums []int) int {
	low, high := 0, len(nums)-1
	for low < high {
		pivot := low + (high-low)/2
		if nums[pivot] < nums[high] {
			high = pivot
		} else {
			low = pivot + 1
		}
	}
	return nums[low]
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{3, 4, 5, 1, 2}, 1},
		{[]int{4, 5, 6, 7, 0, 1, 2}, 0},
		{[]int{11, 13, 15, 17}, 11},
	}

	for _, test := range tests {
		ans := findMin(test.nums)
		if ans != test.ans {
			panic("error")
		}
	}
}
