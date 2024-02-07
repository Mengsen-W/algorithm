/*
 * @Date: 2024-02-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-07
 * @FilePath: /algorithm/golang/2641_replace_value_in_tree/replace_value_in_tree.go
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

func replaceValueInTree(root *TreeNode) *TreeNode {
	q := []*TreeNode{root}
	root.Val = 0
	for len(q) > 0 {
		var q2 []*TreeNode
		sum := 0
		for _, fa := range q {
			if fa.Left != nil {
				q2 = append(q2, fa.Left)
				sum += fa.Left.Val
			}
			if fa.Right != nil {
				q2 = append(q2, fa.Right)
				sum += fa.Right.Val
			}
		}
		for _, fa := range q {
			childSum := 0
			if fa.Left != nil {
				childSum += fa.Left.Val
			}
			if fa.Right != nil {
				childSum += fa.Right.Val
			}
			if fa.Left != nil {
				fa.Left.Val = sum - childSum
			}
			if fa.Right != nil {
				fa.Right.Val = sum - childSum
			}
		}
		q = q2
	}
	return root
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

func main() {
	tests := []struct {
		root *TreeNode
		ans  *TreeNode
	}{
		{
			&TreeNode{
				5, &TreeNode{4, &TreeNode{1, nil, nil}, &TreeNode{10, nil, nil}},
				&TreeNode{9, nil, &TreeNode{7, nil, nil}},
			},
			&TreeNode{
				0, &TreeNode{0, &TreeNode{7, nil, nil}, &TreeNode{7, nil, nil}},
				&TreeNode{0, nil, &TreeNode{11, nil, nil}},
			},
		},
		{
			&TreeNode{3, &TreeNode{1, nil, nil}, &TreeNode{2, nil, nil}},
			&TreeNode{0, &TreeNode{0, nil, nil}, &TreeNode{0, nil, nil}},
		},
	}

	for index, test := range tests {
		assert.True(&testing.T{}, isSameTree(test.ans, replaceValueInTree(test.root)), index)
	}
}
