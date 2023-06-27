/*
 * @Date: 2023-06-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-16
 * @FilePath: /algorithm/golang/1494_min_number_of_semesters/min_number_of_semesters.go
 */

// Package main ...
package main

import (
	"math"
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minNumberOfSemesters(n int, relations [][]int, k int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	dp := make([]int, 1<<n)
	for i := range dp {
		dp[i] = math.MaxInt32
	}
	need := make([]int, 1<<n)
	for _, edge := range relations {
		need[1<<(edge[1]-1)] |= 1 << (edge[0] - 1)
	}
	dp[0] = 0
	for i := 1; i < (1 << n); i++ {
		need[i] = need[i&(i-1)] | need[i&-i]
		if (need[i] | i) != i { // i 中有任意一门课程的前置课程没有完成学习
			continue
		}
		valid := i ^ need[i]                  // 当前学期可以进行学习的课程集合
		if bits.OnesCount(uint(valid)) <= k { // 如果个数小于 k，则可以全部学习，不再枚举子集
			dp[i] = min(dp[i], dp[i^valid]+1)
		} else {
			for sub := valid; sub > 0; sub = (sub - 1) & valid {
				if bits.OnesCount(uint(sub)) <= k {
					dp[i] = min(dp[i], dp[i^sub]+1)
				}
			}
		}
	}
	return dp[(1<<n)-1]
}

func main() {
	{
		n := 4
		relations := [][]int{{2, 1}, {3, 1}, {1, 4}}
		k := 2
		ans := 3
		assert.Equal(&testing.B{}, minNumberOfSemesters(n, relations, k), ans)
	}

	{
		n := 5
		relations := [][]int{{2, 1}, {3, 1}, {4, 1}, {1, 5}}
		k := 2
		ans := 4
		assert.Equal(&testing.B{}, minNumberOfSemesters(n, relations, k), ans)
	}

	{
		n := 11
		relations := [][]int{}
		k := 2
		ans := 6
		assert.Equal(&testing.B{}, minNumberOfSemesters(n, relations, k), ans)
	}
}
