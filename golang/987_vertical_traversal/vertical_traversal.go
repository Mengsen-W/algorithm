/*
 * @Date: 2021-07-31 00:49:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-31 01:44:30
 */

package main

import (
	"math"
	"reflect"
	"sort"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type data struct{ col, row, val int }

func verticalTraversal(root *TreeNode) (ans [][]int) {
	nodes := []data{}
	var dfs func(*TreeNode, int, int)
	dfs = func(node *TreeNode, row, col int) {
		if node == nil {
			return
		}
		nodes = append(nodes, data{col, row, node.Val})
		dfs(node.Left, row+1, col-1)
		dfs(node.Right, row+1, col+1)
	}
	dfs(root, 0, 0)

	sort.Slice(nodes, func(i, j int) bool {
		a, b := nodes[i], nodes[j]
		return a.col < b.col || a.col == b.col && (a.row < b.row || a.row == b.row && a.val < b.val)
	})

	lastCol := math.MinInt32
	for _, node := range nodes {
		if node.col != lastCol {
			lastCol = node.col
			ans = append(ans, nil)
		}
		ans[len(ans)-1] = append(ans[len(ans)-1], node.val)
	}
	return
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		root := &TreeNode{3, &TreeNode{9, nil, nil}, &TreeNode{20, &TreeNode{15, nil, nil}, &TreeNode{7, nil, nil}}}
		ans := [][]int{{9}, {3, 15}, {20}, {7}}
		assert(verticalTraversal(root), ans)
	}
	{
		root := &TreeNode{1, &TreeNode{2, &TreeNode{4, nil, nil}, &TreeNode{5, nil, nil}}, &TreeNode{3, &TreeNode{6, nil, nil}, &TreeNode{7, nil, nil}}}
		ans := [][]int{{4}, {2}, {1, 5, 6}, {3}, {7}}
		assert(verticalTraversal(root), ans)
	}
	{
		root := &TreeNode{1, &TreeNode{2, &TreeNode{4, nil, nil}, &TreeNode{6, nil, nil}}, &TreeNode{3, &TreeNode{5, nil, nil}, &TreeNode{7, nil, nil}}}
		ans := [][]int{{4}, {2}, {1, 5, 6}, {3}, {7}}
		assert(verticalTraversal(root), ans)
	}
}
