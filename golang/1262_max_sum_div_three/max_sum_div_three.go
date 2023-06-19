/*
 * @Date: 2023-06-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-19
 * @FilePath: /algorithm/golang/1262_max_sum_div_three/max_sum_div_three.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/go-playground/assert/v2"
)

func maxSumDivThree(nums []int) int {
	f := []int{0, -0x3f3f3f3f, -0x3f3f3f3f}
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	for _, num := range nums {
		g := make([]int, 3)
		for i := 0; i < 3; i++ {
			g[(i+num)%3] = max(f[(i+num)%3], f[i]+num)
		}
		f = g
	}
	return f[0]
}

func main() {
	{
		nums := []int{3, 6, 5, 1, 8}
		ans := 18
		assert.Equal(&testing.B{}, maxSumDivThree(nums), ans)
	}

	{
		nums := []int{4}
		ans := 0
		assert.Equal(&testing.B{}, maxSumDivThree(nums), ans)
	}

	{
		nums := []int{1, 2, 3, 4, 4}
		ans := 12
		assert.Equal(&testing.B{}, maxSumDivThree(nums), ans)
	}
}
