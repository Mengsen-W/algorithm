// Package main ...
package main

import "fmt"

func getMinDistance(nums []int, target int, start int) int {
	for k := 0; ; k++ {
		if start >= k && nums[start-k] == target ||
			start+k < len(nums) && nums[start+k] == target {
			return k
		}
	}
}

func main() {
	tests := []struct {
		nums   []int
		target int
		start  int
		ans    int
	}{
		{[]int{1, 2, 3, 4, 5}, 5, 3, 1},
		{[]int{1}, 1, 0, 0},
		{[]int{1, 1, 1, 1, 1, 1, 1, 1, 1, 1}, 1, 0, 0},
	}

	for index, test := range tests {
		if getMinDistance(test.nums, test.target, test.start) != test.ans {
			fmt.Errorf("%d", index)
		}
	}
}
