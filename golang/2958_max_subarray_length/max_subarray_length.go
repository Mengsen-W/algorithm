// Package main ...
package main

func maxSubarrayLength(nums []int, k int) int {
	n := len(nums)
	occ := make(map[int]int)
	right := -1
	ans := 0

	for left := 0; left < n; left++ {
		if left > 0 {
			occ[nums[left-1]]--
			if occ[nums[left-1]] == 0 {
				delete(occ, nums[left-1])
			}
		}

		for right+1 < n && occ[nums[right+1]] < k {
			right++
			occ[nums[right]]++
		}

		if right-left+1 > ans {
			ans = right - left + 1
		}
	}

	return ans
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{1, 2, 3, 1, 2, 3, 1, 2}, 2, 6},
		{[]int{1, 2, 1, 2, 1, 2, 1, 2}, 1, 2},
		{[]int{5, 5, 5, 5, 5, 5, 5}, 4, 4},
	}

	for _, test := range tests {
		ans := maxSubarrayLength(test.nums, test.k)
		if ans != test.ans {
			panic("wrong answer")
		}
	}
}
