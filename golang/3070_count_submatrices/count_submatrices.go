// Package main ...
package main

func countSubmatrices(grid [][]int, k int) int {
	n := len(grid)
	m := len(grid[0])
	cols := make([]int, m)
	res := 0

	for i := 0; i < n; i++ {
		rows := 0
		for j := 0; j < m; j++ {
			cols[j] += grid[i][j]
			rows += cols[j]
			if rows <= k {
				res++
			}
		}
	}

	return res
}

func main() {
	tests := []struct {
		grid [][]int
		k    int
		ans  int
	}{
		{[][]int{{7, 6, 3}, {6, 6, 1}}, 18, 4},
		{[][]int{{7, 2, 9}, {1, 5, 0}, {2, 6, 6}}, 20, 6},
	}

	for _, test := range tests {
		ans := countSubmatrices(test.grid, test.k)
		if ans != test.ans {
			panic("error")
		}
	}
}
