/*
 * @Date: 2023-12-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-04
 * @FilePath: /algorithm/golang/1038_bst_to_gst/bst_to_gst.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// TreeNode Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func getSuccessor(node *TreeNode) *TreeNode {
	succ := node.Right
	for succ.Left != nil && succ.Left != node {
		succ = succ.Left
	}
	return succ
}

func bstToGst(root *TreeNode) *TreeNode {
	sum := 0
	node := root
	for node != nil {
		if node.Right == nil {
			sum += node.Val
			node.Val = sum
			node = node.Left
		} else {
			succ := getSuccessor(node)
			if succ.Left == nil {
				succ.Left = node
				node = node.Right
			} else {
				succ.Left = nil
				sum += node.Val
				node.Val = sum
				node = node.Left
			}
		}
	}
	return root
}

func isSameTree(p *TreeNode, q *TreeNode) bool {
	if p == nil && q == nil {
		return true
	}
	if p == nil || q == nil {
		return false
	}
	if p.Val != q.Val {
		return false
	}
	return isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  *TreeNode
	}{
		{
			&TreeNode{
				4, &TreeNode{1, &TreeNode{0, nil, nil}, &TreeNode{2, nil, &TreeNode{3, nil, nil}}},
				&TreeNode{6, &TreeNode{5, nil, nil}, &TreeNode{7, nil, &TreeNode{8, nil, nil}}},
			},
			&TreeNode{
				30, &TreeNode{36, &TreeNode{36, nil, nil}, &TreeNode{35, nil, &TreeNode{33, nil, nil}}},
				&TreeNode{21, &TreeNode{26, nil, nil}, &TreeNode{15, nil, &TreeNode{8, nil, nil}}},
			},
		},
		{
			&TreeNode{0, nil, &TreeNode{1, nil, nil}},
			&TreeNode{1, nil, &TreeNode{1, nil, nil}},
		},
	}

	for index, test := range tests {
		assert.True(&testing.T{}, isSameTree(bstToGst(test.root), test.ans), index)
	}
}
