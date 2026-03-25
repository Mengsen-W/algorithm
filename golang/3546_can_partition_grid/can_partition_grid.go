// Package main ...
package main

func canPartitionGrid(grid [][]int) bool {
	var total int64 = 0
	m, n := len(grid), len(grid[0])
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			total += int64(grid[i][j])
		}
	}
	for k := 0; k < 2; k++ {
		var sum int64 = 0
		m, n = len(grid), len(grid[0])
		for i := 0; i < m-1; i++ {
			for j := 0; j < n; j++ {
				sum += int64(grid[i][j])
			}
			if sum*2 == total {
				return true
			}
		}
		grid = rotation(grid)
	}
	return false
}

func rotation(grid [][]int) [][]int {
	m, n := len(grid), len(grid[0])
	tmp := make([][]int, n)
	for i := range tmp {
		tmp[i] = make([]int, m)
	}
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			tmp[j][m-1-i] = grid[i][j]
		}
	}
	return tmp
}

func main() {
	tests := []struct {
		grid [][]int
		ans  bool
	}{
		{[][]int{{1, 4}, {2, 3}}, true},
		{[][]int{{1, 3}, {2, 4}}, false},
	}

	for _, test := range tests {
		if ret := canPartitionGrid(test.grid); ret != test.ans {
			panic("error")
		}
	}
}
