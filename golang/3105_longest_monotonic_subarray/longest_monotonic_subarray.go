// Package main ...
package main

import "fmt"

func longestMonotonicSubarray(nums []int) int {
	ans := 1
	i := 0
	n := len(nums)
	for i < n-1 {
		if nums[i+1] == nums[i] {
			i++
			continue
		}
		j := i
		inc := nums[i+1] > nums[i]
		i += 2
		for i < n && nums[i] != nums[i-1] && (nums[i] > nums[i-1]) == inc {
			i++
		}
		ans = max(ans, i-j)
		i--
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 4, 3, 3, 2}, 2},
		{[]int{3, 3, 3, 3}, 1},
		{[]int{3, 2, 1}, 3},
	}

	for index, test := range tests {
		if longestMonotonicSubarray(test.nums) != test.ans {
			panic(fmt.Sprintf("test %d failed", index))
		}
	}
}
