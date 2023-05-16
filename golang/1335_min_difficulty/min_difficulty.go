/*
 * @Date: 2023-05-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-16
 * @FilePath: /algorithm/golang/1335_min_difficulty/min_difficulty.go
 */

// Package main ...
package main

import "math"

func minDifficulty(a []int, d int) int {
	min := func(a, b int) int {
		if b < a {
			return b
		}
		return a
	}
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	n := len(a)
	if n < d {
		return -1
	}

	f := make([]int, n)
	f[0] = a[0]
	for i := 1; i < n; i++ {
		f[i] = max(f[i-1], a[i])
	}
	for i := 1; i < d; i++ {
		for j := n - 1; j >= i; j-- {
			f[j] = math.MaxInt
			mx := 0
			for k := j; k >= i; k-- {
				mx = max(mx, a[k]) // 从 a[k] 到 a[j] 的最大值
				f[j] = min(f[j], f[k-1]+mx)
			}
		}
	}
	return f[n-1]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		jobDifficulty := []int{6, 5, 4, 3, 2, 1}
		d := 2
		ans := 7
		assert(minDifficulty(jobDifficulty, d) == ans)
	}

	{
		jobDifficulty := []int{9, 9, 9}
		d := 4
		ans := -1
		assert(minDifficulty(jobDifficulty, d) == ans)
	}

	{
		jobDifficulty := []int{1, 1, 1}
		d := 3
		ans := 3
		assert(minDifficulty(jobDifficulty, d) == ans)
	}

	{
		jobDifficulty := []int{7, 1, 7, 1, 7, 1}
		d := 3
		ans := 15
		assert(minDifficulty(jobDifficulty, d) == ans)
	}

	{
		jobDifficulty := []int{11, 111, 22, 222, 33, 333, 44, 444}
		d := 6
		ans := 843
		assert(minDifficulty(jobDifficulty, d) == ans)
	}
}
