// Package main ...
package main

func minCost(grid [][]int) int {
	m := len(grid)
	n := len(grid[0])

	dp := make([][1024]bool, n)
	var next [1024]bool

	dp[0][grid[0][0]] = true

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if i == 0 && j == 0 {
				continue
			}

			value := grid[i][j]
			clear(next[:])

			for k := 0; k < 1024; k++ {
				if dp[j][k] || (j > 0 && dp[j-1][k]) {
					next[k^value] = true
				}
			}

			dp[j] = next
		}
	}

	for k, value := range dp[n-1] {
		if value {
			return k
		}
	}

	return -1
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{1, 2}, {3, 4}}, 6},
		{[][]int{{6, 7}, {5, 8}}, 9},
		{[][]int{{2, 7, 5}}, 0},
	}

	for _, test := range tests {
		if ans := minCost(test.grid); ans != test.ans {
			panic("wrong answer")
		}
	}
}
