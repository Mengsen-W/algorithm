/*
 * @Date: 2021-05-21 08:42:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-21 08:52:13
 */

package main

func maxUncrossedLines(nums1, nums2 []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	m, n := len(nums1), len(nums2)
	dp := make([][]int, m+1)
	for i := range dp {
		dp[i] = make([]int, n+1)
	}
	for i, v := range nums1 {
		for j, w := range nums2 {
			if v == w {
				dp[i+1][j+1] = dp[i][j] + 1
			} else {
				dp[i+1][j+1] = max(dp[i][j+1], dp[i+1][j])
			}
		}
	}
	return dp[m][n]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(maxUncrossedLines([]int{1, 4, 2}, []int{1, 2, 4}) == 2)
	assert(maxUncrossedLines([]int{2, 5, 1, 2, 5}, []int{10, 5, 2, 1, 5, 2}) == 3)
	assert(maxUncrossedLines([]int{1, 3, 7, 1, 7, 5}, []int{1, 9, 2, 5, 1}) == 2)
}
