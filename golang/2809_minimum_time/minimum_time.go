/*
 * @Date: 2024-01-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-19
 * @FilePath: /algorithm/golang/2809_minimum_time/minimum_time.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumTime(nums1 []int, nums2 []int, x int) int {
	n := len(nums1)
	dp := make([]int, n+1)

	nums := make([][2]int, n)
	for i := 0; i < n; i++ {
		nums[i] = [2]int{nums2[i], nums1[i]}
	}
	sort.Slice(nums, func(i, j int) bool {
		return nums[i][0] < nums[j][0]
	})

	for j := 1; j <= n; j++ {
		b, a := nums[j-1][0], nums[j-1][1]
		for i := j; i > 0; i-- {
			dp[i] = max(dp[i], dp[i-1]+i*b+a)
		}
	}

	s1 := 0
	for _, v := range nums1 {
		s1 += v
	}
	s2 := 0
	for _, v := range nums2 {
		s2 += v
	}
	for i := 0; i <= n; i++ {
		if s2*i+s1-dp[i] <= x {
			return i
		}
	}
	return -1
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		x     int
		ans   int
	}{
		{[]int{1, 2, 3}, []int{1, 2, 3}, 4, 3},
		{[]int{1, 2, 3}, []int{3, 3, 3}, 4, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumTime(test.nums1, test.nums2, test.x), index)
	}
}
