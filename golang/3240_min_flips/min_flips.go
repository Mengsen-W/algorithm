// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minFlips(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	f := make([]int, 4)
	for i := range f {
		f[i] = math.MaxInt32 / 2
	}
	f[0] = 0
	for i := 0; i < (m+1)/2; i++ {
		for j := 0; j < (n+1)/2; j++ {
			ones := grid[i][j]
			cnt := 1
			if j != n-1-j {
				ones += grid[i][n-1-j]
				cnt++
			}
			if i != m-1-i {
				ones += grid[m-1-i][j]
				cnt++
			}
			if i != m-1-i && j != n-1-j {
				ones += grid[m-1-i][n-1-j]
				cnt++
			}
			// 计算将这一组全部变为 1 的代价
			cnt1 := cnt - ones
			// 计算将这一组全部变为 0 的代价
			cnt0 := ones
			tmp := make([]int, 4)
			for k := 0; k < 4; k++ {
				tmp[k] = f[k] + cnt0
			}
			for k := 0; k < 4; k++ {
				tmp[(k+cnt)%4] = min(tmp[(k+cnt)%4], f[k]+cnt1)
			}
			f = tmp
		}
	}
	return f[0]
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{1, 0, 0}, {0, 1, 0}, {0, 0, 1}}, 3},
		{[][]int{{0, 1}, {0, 1}, {0, 0}}, 2},
		{[][]int{{1}, {1}}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minFlips(test.grid), index)
	}
}
