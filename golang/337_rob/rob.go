/*
 * @Date: 2023-09-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-18
 * @FilePath: /algorithm/golang/337_rob/rob.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rob(root *TreeNode) int {
	val := dfs(root)
	return max(val[0], val[1])
}

func dfs(node *TreeNode) []int {
	if node == nil {
		return []int{0, 0}
	}
	l, r := dfs(node.Left), dfs(node.Right)
	selected := node.Val + l[1] + r[1]
	notSelected := max(l[0], l[1]) + max(r[0], r[1])
	return []int{selected, notSelected}
}

func max(x, y int) int {
	if x > y {
		return x
	}
	return y
}

func main() {
	tests := []struct {
		node *TreeNode
		ans  int
	}{
		{
			&TreeNode{3, &TreeNode{2, &TreeNode{3, nil, nil}, nil}, &TreeNode{3, nil, &TreeNode{1, nil, nil}}},
			7,
		},
		{
			&TreeNode{3, &TreeNode{4, &TreeNode{1, nil, nil}, &TreeNode{3, nil, nil}}, &TreeNode{5, nil, &TreeNode{1, nil, nil}}},
			9,
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, rob(test.node), index)
	}
}
