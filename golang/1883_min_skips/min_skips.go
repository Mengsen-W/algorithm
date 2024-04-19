/*
 * @Date: 2024-04-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-19
 * @FilePath: /algorithm/golang/1883_min_skips/min_skips.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minSkips(dist []int, speed int, hoursBefore int) int {
	// 可忽略误差
	const EPS = 1e-7

	n := len(dist)
	f := make([][]float64, n+1)
	for i := range f {
		f[i] = make([]float64, n+1)
		for j := range f[i] {
			f[i][j] = math.Inf(1)
		}
	}
	f[0][0] = 0.0
	for i := 1; i <= n; i++ {
		for j := 0; j <= i; j++ {
			if j != i {
				f[i][j] = math.Min(f[i][j], math.Ceil(f[i-1][j]+float64(dist[i-1])/float64(speed)-EPS))
			}
			if j != 0 {
				f[i][j] = math.Min(f[i][j], f[i-1][j-1]+float64(dist[i-1])/float64(speed))
			}
		}
	}

	for j := 0; j <= n; j++ {
		if f[n][j] < float64(hoursBefore)+EPS {
			return j
		}
	}
	return -1
}

func main() {
	tests := []struct {
		dist        []int
		speed       int
		hoursBefore int
		ans         int
	}{
		{[]int{1, 3, 2}, 4, 2, 1},
		{[]int{7, 3, 5, 5}, 2, 10, 2},
		{[]int{7, 3, 5, 5}, 1, 10, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minSkips(test.dist, test.speed, test.hoursBefore), index)
	}
}
