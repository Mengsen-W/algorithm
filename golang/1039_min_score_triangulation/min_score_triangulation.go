/*
 * @Date: 2023-04-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-02
 * @FilePath: /algorithm/golang/1039_min_score_triangulation/min_score_triangulation.go
 */

// Package main
package main

import (
	"math"
)

func minScoreTriangulation(values []int) int {
	memo := make(map[int]int)
	n := len(values)
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	var dp func(int, int) int
	dp = func(i int, j int) int {
		if i+2 > j {
			return 0
		}
		if i+2 == j {
			return values[i] * values[i+1] * values[j]
		}
		key := i*n + j
		if _, ok := memo[key]; !ok {
			minScore := math.MaxInt32
			for k := i + 1; k < j; k++ {
				minScore = min(minScore, values[i]*values[k]*values[j]+dp(i, k)+dp(k, j))
			}
			memo[key] = minScore
		}
		return memo[key]
	}
	return dp(0, n-1)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		values := []int{1, 2, 3}
		ans := 6
		assert(minScoreTriangulation(values) == ans)
	}

	{
		values := []int{3, 7, 4, 5}
		ans := 144
		assert(minScoreTriangulation(values) == ans)
	}

	{
		values := []int{1, 3, 1, 4, 1, 5}
		ans := 13
		assert(minScoreTriangulation(values) == ans)
	}
}
