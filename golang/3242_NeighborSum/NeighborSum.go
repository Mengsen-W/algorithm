package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type NeighborSum struct {
	grid [][]int
	pos  map[int][2]int
}

var dirs = [2][4][2]int{
	{{-1, 0}, {1, 0}, {0, -1}, {0, 1}},
	{{-1, -1}, {-1, 1}, {1, -1}, {1, 1}},
}

func Constructor(grid [][]int) NeighborSum {
	this := NeighborSum{
		grid: grid,
		pos:  make(map[int][2]int),
	}
	for i := range grid {
		for j := range grid[0] {
			this.pos[grid[i][j]] = [2]int{i, j}
		}
	}

	return this
}

func (n *NeighborSum) AdjacentSum(value int) int {
	return n.getSum(value, 0)
}

func (n *NeighborSum) DiagonalSum(value int) int {
	return n.getSum(value, 1)
}

func (n *NeighborSum) getSum(value, idx int) int {
	pos := n.pos[value]
	x, y := pos[0], pos[1]
	sum := 0
	for _, dir := range dirs[idx] {
		nx, ny := x+dir[0], y+dir[1]
		if nx >= 0 && nx < len(n.grid) && ny >= 0 && ny < len(n.grid[0]) {
			sum += n.grid[nx][ny]
		}
	}
	return sum
}

func main() {
	{
		n := Constructor([][]int{{0, 1, 2}, {3, 4, 5}, {6, 7, 8}})
		assert.Equal(&testing.T{}, 6, n.AdjacentSum(1))
		assert.Equal(&testing.T{}, 16, n.AdjacentSum(4))
		assert.Equal(&testing.T{}, 16, n.DiagonalSum(4))
		assert.Equal(&testing.T{}, 4, n.DiagonalSum(8))
	}

	{
		n := Constructor([][]int{{1, 2, 0, 3}, {4, 7, 15, 6}, {8, 9, 10, 11}, {12, 13, 14, 5}})
		assert.Equal(&testing.T{}, 23, n.AdjacentSum(15))
		assert.Equal(&testing.T{}, 45, n.DiagonalSum(9))
	}
}
