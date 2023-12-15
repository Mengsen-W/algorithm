/*
 * @Date: 2023-12-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-15
 * @FilePath: /algorithm/golang/2415_reverse_odd_levels/reverse_odd_levels.go
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

func isSameTree(p, q *TreeNode) bool {
	if p == nil || q == nil {
		return p == q
	}

	if p.Val != q.Val {
		return false
	}

	return isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}

func reverseOddLevels(root *TreeNode) *TreeNode {
	q := []*TreeNode{root}
	isOdd := 0
	for len(q) > 0 {
		if isOdd != 0 {
			n := len(q)
			for i := 0; i < n/2; i++ {
				nodex, nodey := q[i], q[n-1-i]
				nodex.Val, nodey.Val = nodey.Val, nodex.Val
			}
		}
		tmp := make([]*TreeNode, 0, len(q)*2)
		for _, node := range q {
			if node.Left != nil {
				tmp = append(tmp, node.Left, node.Right)
			}
		}
		q = tmp
		isOdd ^= 1
	}
	return root
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  *TreeNode
	}{
		{
			&TreeNode{
				2, &TreeNode{3, &TreeNode{8, nil, nil}, &TreeNode{13, nil, nil}},
				&TreeNode{5, &TreeNode{21, nil, nil}, &TreeNode{34, nil, nil}},
			},
			&TreeNode{
				2, &TreeNode{5, &TreeNode{8, nil, nil}, &TreeNode{13, nil, nil}},
				&TreeNode{3, &TreeNode{21, nil, nil}, &TreeNode{34, nil, nil}},
			},
		},
		{
			&TreeNode{7, &TreeNode{13, nil, nil}, &TreeNode{11, nil, nil}},
			&TreeNode{7, &TreeNode{11, nil, nil}, &TreeNode{13, nil, nil}},
		},
		{
			&TreeNode{
				0,
				&TreeNode{
					1, &TreeNode{0, &TreeNode{1, nil, nil}, &TreeNode{1, nil, nil}},
					&TreeNode{0, &TreeNode{1, nil, nil}, &TreeNode{1, nil, nil}},
				},
				&TreeNode{
					2, &TreeNode{0, &TreeNode{2, nil, nil}, &TreeNode{2, nil, nil}},
					&TreeNode{0, &TreeNode{2, nil, nil}, &TreeNode{2, nil, nil}},
				},
			},
			&TreeNode{
				0,
				&TreeNode{
					2, &TreeNode{0, &TreeNode{2, nil, nil}, &TreeNode{2, nil, nil}},
					&TreeNode{0, &TreeNode{2, nil, nil}, &TreeNode{2, nil, nil}},
				},
				&TreeNode{
					1, &TreeNode{0, &TreeNode{1, nil, nil}, &TreeNode{1, nil, nil}},
					&TreeNode{0, &TreeNode{1, nil, nil}, &TreeNode{1, nil, nil}},
				},
			},
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, isSameTree(test.ans, reverseOddLevels(test.root)), true, index)
	}
}
