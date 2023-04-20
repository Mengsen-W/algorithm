/*
 * @Date: 2023-04-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-20
 * @FilePath: /algorithm/golang/1187_make_array_increasing/make_array_increasing.go
 */

// Package main ...
package main

import (
	"math"
	"sort"
)

func makeArrayIncreasing(arr1 []int, arr2 []int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	sort.Ints(arr2)
	n := len(arr1)
	m := len(arr2)
	dp := make([][]int, n+1)
	for i := range dp {
		dp[i] = make([]int, min(m, n)+1)
		for j := range dp[i] {
			dp[i][j] = math.MaxInt
		}
	}
	dp[0][0] = -1
	for i := 1; i <= len(arr1); i++ {
		for j := 0; j <= min(i, m); j++ {
			if arr1[i-1] > dp[i-1][j] {
				dp[i][j] = arr1[i-1]
			}
			if j > 0 && dp[i-1][j-1] != math.MaxInt {
				k := j - 1 + sort.SearchInts(arr2[j-1:], dp[i-1][j-1]+1)
				if k < m {
					dp[i][j] = min(dp[i][j], arr2[k])
				}
			}
			if i == n && dp[i][j] != math.MaxInt {
				return j
			}
		}
	}
	return -1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		arr1 := []int{1, 5, 3, 5, 7}
		arr2 := []int{1, 3, 2, 4}
		ans := 1
		assert(makeArrayIncreasing(arr1, arr2) == ans)
	}

	{
		arr1 := []int{1, 5, 3, 6, 7}
		arr2 := []int{4, 3, 1}
		ans := 2
		assert(makeArrayIncreasing(arr1, arr2) == ans)
	}

	{
		arr1 := []int{1, 5, 3, 5, 7}
		arr2 := []int{1, 6, 3, 3}
		ans := -1
		assert(makeArrayIncreasing(arr1, arr2) == ans)
	}
}
