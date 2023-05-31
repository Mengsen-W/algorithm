/*
 * @Date: 2023-05-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-31
 * @FilePath: /algorithm/golang/1130_mct_from_leaf_values/mct_from_leaf_values.go
 */

// Package main ...
package main

func mctFromLeafValues(arr []int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	n := len(arr)
	dp, mval := make([][]int, n), make([][]int, n)
	for i := 0; i < n; i++ {
		dp[i] = make([]int, n)
		mval[i] = make([]int, n)
	}
	for j := 0; j < n; j++ {
		mval[j][j] = arr[j]
		for i := j - 1; i >= 0; i-- {
			mval[i][j] = max(arr[i], mval[i+1][j])
			dp[i][j] = 0x3f3f3f3f
			for k := i; k < j; k++ {
				dp[i][j] = min(dp[i][j], dp[i][k]+dp[k+1][j]+mval[i][k]*mval[k+1][j])
			}
		}
	}
	return dp[0][n-1]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		arr := []int{6, 2, 4}
		ans := 32
		assert(mctFromLeafValues(arr) == ans)
	}

	{
		arr := []int{4, 11}
		ans := 44
		assert(mctFromLeafValues(arr) == ans)
	}
}
