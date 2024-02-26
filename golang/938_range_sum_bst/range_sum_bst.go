/*
 * @Date: 2024-02-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-26
 * @FilePath: /algorithm/golang/938_range_sum_bst/range_sum_bst.go
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

func rangeSumBST(root *TreeNode, low, high int) (sum int) {
	q := []*TreeNode{root}
	for len(q) > 0 {
		node := q[0]
		q = q[1:]
		if node == nil {
			continue
		}
		if node.Val > high {
			q = append(q, node.Left)
		} else if node.Val < low {
			q = append(q, node.Right)
		} else {
			sum += node.Val
			q = append(q, node.Left, node.Right)
		}
	}
	return
}

func main() {
	tests := []struct {
		root *TreeNode
		low  int
		high int
		ans  int
	}{
		{
			&TreeNode{
				10, &TreeNode{5, &TreeNode{3, nil, nil}, &TreeNode{7, nil, nil}},
				&TreeNode{15, nil, &TreeNode{18, nil, nil}},
			},
			7,
			15,
			32,
		},
		{
			&TreeNode{
				10,
				&TreeNode{
					5, &TreeNode{3, &TreeNode{3, nil, nil}, nil},
					&TreeNode{7, &TreeNode{6, nil, nil}, nil},
				},
				&TreeNode{15, &TreeNode{13, nil, nil}, &TreeNode{18, nil, nil}},
			},
			6,
			10,
			23,
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, rangeSumBST(test.root, test.low, test.high), index)
	}
}
