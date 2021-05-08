/*
 * @Date: 2021-05-08 08:54:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-08 09:22:18
 */

package main

import (
	"fmt"
	"math"
	"math/bits"
)

func minimumTimeRequired(jobs []int, k int) int {
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
	n := len(jobs)
	m := 1 << n
	sum := make([]int, m)
	for i := 1; i < m; i++ {
		x := bits.TrailingZeros(uint(i))
		y := i ^ 1<<x
		sum[i] = sum[y] + jobs[x]
	}

	dp := make([][]int, k)
	for i := range dp {
		dp[i] = make([]int, m)
	}
	for i, s := range sum {
		dp[0][i] = s
	}

	for i := 1; i < k; i++ {
		for j := 0; j < (1 << n); j++ {
			minn := math.MaxInt64
			for x := j; x > 0; x = (x - 1) & j {
				minn = min(minn, max(dp[i-1][j-x], sum[x]))
			}
			dp[i][j] = minn
		}
	}
	return dp[k-1][(1<<n)-1]
}

func main() {
	{
		job := [...]int{3, 2, 3}
		if minimumTimeRequired(job[:], 3) != 3 {
			fmt.Printf("Not Passed!")
		}

	}
	{
		job := [...]int{1, 2, 4, 7, 8}
		if minimumTimeRequired(job[:], 2) != 11 {
			fmt.Printf("Not Passed!")
		}

	}
}
