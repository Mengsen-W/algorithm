/*
 * @Date: 2023-08-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-25
 * @FilePath: /algorithm/golang/1448_good_nodes/good_nodes.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

// TreeNode Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func goodNodes(root *TreeNode) int {
	var dfs func(root *TreeNode, m int) int
	dfs = func(root *TreeNode, m int) int {
		if root == nil {
			return 0
		}
		res := 0
		if root.Val >= m {
			res++
			m = root.Val
		}
		return res + dfs(root.Left, m) + dfs(root.Right, m)
	}
	return dfs(root, math.MinInt)
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  int
	}{
		{
			&TreeNode{3, &TreeNode{1, &TreeNode{3, nil, nil}, nil}, &TreeNode{4, &TreeNode{1, nil, nil}, &TreeNode{5, nil, nil}}},
			4,
		},
		{
			&TreeNode{3, &TreeNode{3, &TreeNode{4, nil, nil}, &TreeNode{2, nil, nil}}, nil},
			3,
		},
		{
			&TreeNode{3, nil, nil},
			1,
		},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, goodNodes(item.root))
	}
}
