// Package main ...
package main

import "fmt"

func minimumDistance(nums []int) int {
	n := len(nums)
	ans := n + 1

	for i := 0; i < n-2; i++ {
		for j := i + 1; j < n-1; j++ {
			if nums[i] != nums[j] {
				continue
			}
			for k := j + 1; k < n; k++ {
				if nums[j] == nums[k] {
					if dist := k - i; dist < ans {
						ans = dist
					}
					break
				}
			}
		}
	}

	if ans == n+1 {
		return -1
	}
	return ans * 2
}

func main() {
	tests := []struct{
		nums []int
		ans int
	}{
      {[]int{1, 2, 1, 1, 3}, 6},
      {[]int{1, 1, 2, 3, 2, 1, 2}, 8},
      {[]int{1}, -1},
	}

	for index, test := range tests {
		if minimumDistance(test.nums) != test.ans {
			fmt.Errorf("%d", index)
		}
	}
}
