// Package main ...
package main

import "fmt"

func countMajoritySubarrays(nums []int, target int) int {
	n := len(nums)
	ans := 0
	for i := 0; i < n; i++ {
		cnt := 0
		for j := i; j < n; j++ {
			if nums[j] == target {
				cnt++
			} else {
				cnt--
			}
			if cnt > 0 {
				ans++
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums   []int
		target int
		ans    int
	}{
		{[]int{1, 2, 2, 3}, 2, 5},
		{[]int{1, 1, 1, 1}, 1, 10},
		{[]int{1, 2, 3}, 4, 0},
	}

	for _, test := range tests {
		got := countMajoritySubarrays(test.nums, test.target)
		if got != test.ans {
			fmt.Printf("countMajoritySubarrays(%v, %d) = %d, want %d\n", test.nums, test.target, got, test.ans)
		}
	}
}
