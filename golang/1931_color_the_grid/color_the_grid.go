// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

const mod = 1000000007

func colorTheGrid(m int, n int) int {
	// 哈希映射 valid 存储所有满足要求的对一行进行涂色的方案
	valid := make(map[int][]int)

	// 在 [0, 3^m) 范围内枚举满足要求的 mask
	maskEnd := int(math.Pow(3, float64(m)))
	for mask := 0; mask < maskEnd; mask++ {
		color := make([]int, m)
		mm := mask
		for i := 0; i < m; i++ {
			color[i] = mm % 3
			mm /= 3
		}
		check := true
		for i := 0; i < m-1; i++ {
			if color[i] == color[i+1] {
				check = false
				break
			}
		}
		if check {
			valid[mask] = color
		}
	}

	// 预处理所有的 (mask1, mask2) 二元组，满足 mask1 和 mask2 作为相邻行时，同一列上两个格子的颜色不同
	adjacent := make(map[int][]int)
	for mask1 := range valid {
		for mask2 := range valid {
			check := true
			for i := 0; i < m; i++ {
				if valid[mask1][i] == valid[mask2][i] {
					check = false
					break
				}
			}
			if check {
				adjacent[mask1] = append(adjacent[mask1], mask2)
			}
		}
	}

	f := make(map[int]int)
	for mask := range valid {
		f[mask] = 1
	}
	for i := 1; i < n; i++ {
		g := make(map[int]int)
		for mask2 := range valid {
			for _, mask1 := range adjacent[mask2] {
				g[mask2] = (g[mask2] + f[mask1]) % mod
			}
		}
		f = g
	}

	ans := 0
	for _, num := range f {
		ans = (ans + num) % mod
	}
	return ans
}

func main() {
	tests := []struct {
		m   int
		n   int
		ans int
	}{
		{1, 1, 3},
		{1, 2, 6},
		{5, 5, 580986},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, colorTheGrid(test.m, test.n), index)
	}
}
