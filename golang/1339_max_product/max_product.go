// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxProduct(root *TreeNode) int {
	sum := 0
	best := 0

	var dfs func(*TreeNode)
	dfs = func(node *TreeNode) {
		if node == nil {
			return
		}
		sum += node.Val
		dfs(node.Left)
		dfs(node.Right)
	}

	var dfs2 func(*TreeNode) int
	dfs2 = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		cur := dfs2(node.Left) + dfs2(node.Right) + node.Val
		if abs(cur*2-sum) < abs(best*2-sum) {
			best = cur
		}
		return cur
	}

	dfs(root)
	dfs2(root)
	return best * (sum - best) % 1000000007
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  int
	}{
		{
			&TreeNode{1, &TreeNode{2, &TreeNode{4, nil, nil}, &TreeNode{5, nil, nil}}, &TreeNode{3, &TreeNode{6, nil, nil}, nil}},
			110,
		},
		{
			&TreeNode{1, nil, &TreeNode{2, &TreeNode{3, nil, nil}, &TreeNode{4, &TreeNode{5, nil, nil}, &TreeNode{6, nil, nil}}}},
			90,
		},
		{
			&TreeNode{
				2,
				&TreeNode{
					3, &TreeNode{10, &TreeNode{5, nil, nil}, &TreeNode{4, nil, nil}},
					&TreeNode{7, &TreeNode{11, nil, nil}, &TreeNode{1, nil, nil}},
				},
				&TreeNode{9, &TreeNode{8, nil, nil}, &TreeNode{6, nil, nil}},
			},
			1025,
		},
		{
			&TreeNode{1, &TreeNode{1, nil, nil}, nil},
			1,
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxProduct(test.root), index)
	}
}
