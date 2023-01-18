/*
 * @Date: 2022-04-29 09:29:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-29 09:51:23
 * @FilePath: /algorithm/427_construct_quad_tree/construct_quad_tree.go
 */

package main

// Definition for a QuadTree node.
type Node struct {
	Val         bool
	IsLeaf      bool
	TopLeft     *Node
	TopRight    *Node
	BottomLeft  *Node
	BottomRight *Node
}

func construct(grid [][]int) *Node {
	n := len(grid)
	pre := make([][]int, n+1)
	pre[0] = make([]int, n+1)
	for i, row := range grid {
		pre[i+1] = make([]int, n+1)
		for j, v := range row {
			pre[i+1][j+1] = pre[i+1][j] + pre[i][j+1] - pre[i][j] + v
		}
	}

	var dfs func(r0, c0, r1, c1 int) *Node
	dfs = func(r0, c0, r1, c1 int) *Node {
		total := pre[r1][c1] - pre[r1][c0] - pre[r0][c1] + pre[r0][c0]
		if total == 0 {
			return &Node{Val: false, IsLeaf: true}
		}
		if total == (r1-r0)*(c1-c0) {
			return &Node{Val: true, IsLeaf: true}
		}
		rMid, cMid := (r0+r1)/2, (c0+c1)/2
		return &Node{
			true,
			false,
			dfs(r0, c0, rMid, cMid),
			dfs(r0, cMid, rMid, c1),
			dfs(rMid, c0, r1, cMid),
			dfs(rMid, cMid, r1, c1),
		}
	}
	return dfs(0, 0, n, n)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{

		grid := [][]int{{0, 1}, {1, 0}}
		output := construct(grid)
		assert(!output.IsLeaf)
	}
}
