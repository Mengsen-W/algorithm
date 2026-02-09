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

func balanceBST(root *TreeNode) *TreeNode {
	var inorderSeq []int

	var getInorder func(*TreeNode)
	getInorder = func(o *TreeNode) {
		if o.Left != nil {
			getInorder(o.Left)
		}
		inorderSeq = append(inorderSeq, o.Val)
		if o.Right != nil {
			getInorder(o.Right)
		}
	}

	var build func(int, int) *TreeNode
	build = func(l, r int) *TreeNode {
		mid := (l + r) >> 1
		o := &TreeNode{Val: inorderSeq[mid]}
		if l <= mid-1 {
			o.Left = build(l, mid-1)
		}
		if mid+1 <= r {
			o.Right = build(mid+1, r)
		}
		return o
	}

	getInorder(root)
	return build(0, len(inorderSeq)-1)
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
		input  *TreeNode
		output *TreeNode
	}{
		{
			&TreeNode{1, nil, &TreeNode{2, nil, &TreeNode{3, nil, &TreeNode{4, nil, nil}}}},
			&TreeNode{2, &TreeNode{1, nil, nil}, &TreeNode{3, nil, &TreeNode{4, nil, nil}}},
		},
		{
			&TreeNode{2, &TreeNode{1, nil, nil}, &TreeNode{3, nil, nil}},
			&TreeNode{2, &TreeNode{1, nil, nil}, &TreeNode{3, nil, nil}},
		},
	}

	for index, test := range tests {
		assert.True(&testing.T{}, isSameTree(test.output, balanceBST(test.input)), "test %d", index)
	}
}
