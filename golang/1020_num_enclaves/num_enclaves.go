/*
 * @Date: 2022-02-12 00:02:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-12 00:31:42
 */

package main

func numEnclavesDfs(grid [][]int) (ans int) {
	m, n := len(grid), len(grid[0])
	vis := make([][]bool, m)
	for i := range vis {
		vis[i] = make([]bool, n)
	}
	var dirs = []struct{ x, y int }{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	var dfs func(int, int)
	dfs = func(r, c int) {
		if r < 0 || r >= m || c < 0 || c >= n || grid[r][c] == 0 || vis[r][c] {
			return
		}
		vis[r][c] = true
		for _, d := range dirs {
			dfs(r+d.x, c+d.y)
		}
	}
	for i := range grid {
		dfs(i, 0)
		dfs(i, n-1)
	}
	for j := 1; j < n-1; j++ {
		dfs(0, j)
		dfs(m-1, j)
	}
	for i := 1; i < m-1; i++ {
		for j := 1; j < n-1; j++ {
			if grid[i][j] == 1 && !vis[i][j] {
				ans++
			}
		}
	}
	return
}

func numEnclavesBfs(grid [][]int) (ans int) {
	m, n := len(grid), len(grid[0])
	vis := make([][]bool, m)
	for i := range vis {
		vis[i] = make([]bool, n)
	}
	type pair struct{ x, y int }
	var dirs = []pair{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	q := []pair{}
	for i, row := range grid {
		if row[0] == 1 {
			vis[i][0] = true
			q = append(q, pair{i, 0})
		}
		if row[n-1] == 1 {
			vis[i][n-1] = true
			q = append(q, pair{i, n - 1})
		}
	}
	for j := 1; j < n-1; j++ {
		if grid[0][j] == 1 {
			vis[0][j] = true
			q = append(q, pair{0, j})
		}
		if grid[m-1][j] == 1 {
			vis[m-1][j] = true
			q = append(q, pair{m - 1, j})
		}
	}
	for len(q) > 0 {
		p := q[0]
		q = q[1:]
		for _, d := range dirs {
			if x, y := p.x+d.x, p.y+d.y; 0 <= x && x < m && 0 <= y && y < n && grid[x][y] == 1 && !vis[x][y] {
				vis[x][y] = true
				q = append(q, pair{x, y})
			}
		}
	}
	for i := 1; i < m-1; i++ {
		for j := 1; j < n-1; j++ {
			if grid[i][j] == 1 && !vis[i][j] {
				ans++
			}
		}
	}
	return
}

type unionFind struct {
	parent []int
	rank   []int
	onEdge []bool
}

func (uf unionFind) find(x int) int {
	if uf.parent[x] != x {
		uf.parent[x] = uf.find(uf.parent[x])
	}
	return uf.parent[x]
}

func (uf unionFind) merge(x, y int) {
	x, y = uf.find(x), uf.find(y)
	if x == y {
		return
	}
	if uf.rank[x] > uf.rank[y] {
		uf.parent[y] = x
		uf.onEdge[x] = uf.onEdge[x] || uf.onEdge[y]
	} else if uf.rank[x] < uf.rank[y] {
		uf.parent[x] = y
		uf.onEdge[y] = uf.onEdge[y] || uf.onEdge[x]
	} else {
		uf.parent[y] = x
		uf.onEdge[x] = uf.onEdge[x] || uf.onEdge[y]
		uf.rank[x]++
	}
}

func numEnclavesDiffUnion(grid [][]int) (ans int) {
	newUnionFind := func(grid [][]int) unionFind {
		m, n := len(grid), len(grid[0])
		parent := make([]int, m*n)
		rank := make([]int, m*n)
		onEdge := make([]bool, m*n)
		for i, row := range grid {
			for j, v := range row {
				if v == 1 {
					idx := i*n + j
					parent[idx] = idx
					if i == 0 || i == m-1 || j == 0 || j == n-1 {
						onEdge[idx] = true
					}
				}
			}
		}
		return unionFind{parent, rank, onEdge}
	}
	uf := newUnionFind(grid)
	m, n := len(grid), len(grid[0])
	for i, row := range grid {
		for j, v := range row {
			if v == 1 {
				idx := i*n + j
				if j+1 < n && grid[i][j+1] == 1 {
					uf.merge(idx, idx+1)
				}
				if i+1 < m && grid[i+1][j] == 1 {
					uf.merge(idx, idx+n)
				}
			}
		}
	}
	for i := 1; i < m-1; i++ {
		for j := 1; j < n-1; j++ {
			if grid[i][j] == 1 && !uf.onEdge[uf.find(i*n+j)] {
				ans++
			}
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		grid := [][]int{
			{0, 0, 0, 0}, {1, 0, 1, 0}, {0, 1, 1, 0}, {0, 0, 0, 0}}
		assert(numEnclavesBfs(grid) == 3)
		assert(numEnclavesDfs(grid) == 3)
		assert(numEnclavesDiffUnion(grid) == 3)
	}
	{
		grid := [][]int{
			{0, 1, 1, 0}, {0, 0, 1, 0}, {0, 0, 1, 0}, {0, 0, 0, 0}}
		assert(numEnclavesBfs(grid) == 0)
		assert(numEnclavesDfs(grid) == 0)
		assert(numEnclavesDiffUnion(grid) == 0)
	}
}
