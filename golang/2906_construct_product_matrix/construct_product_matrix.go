// Package main ...
package main

import (
	"fmt"
	"reflect"
)

func constructProductMatrix(grid [][]int) [][]int {
	const MOD = 12345
	n, m := len(grid), len(grid[0])
	p := make([][]int, n)
	for i := range p {
		p[i] = make([]int, m)
	}

	suffix := int64(1)
	for i := n - 1; i >= 0; i-- {
		for j := m - 1; j >= 0; j-- {
			p[i][j] = int(suffix)
			suffix = (suffix * int64(grid[i][j])) % MOD
		}
	}

	prefix := int64(1)
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			p[i][j] = int((int64(p[i][j]) * prefix) % MOD)
			prefix = (prefix * int64(grid[i][j])) % MOD
		}
	}

	return p
}

func main() {
	tests := []struct {
		grid [][]int
		ans  [][]int
	}{
		{[][]int{{1, 2}, {3, 4}}, [][]int{{24, 12}, {8, 6}}},
		{[][]int{{12345}, {2}, {1}}, [][]int{{2}, {0}, {0}}},
	}

	for _, test := range tests {
		if ans := constructProductMatrix(test.grid); !reflect.DeepEqual(ans, test.ans) {
			fmt.Println(test.grid, ans, test.ans)
		}
	}
}
