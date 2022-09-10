/*
 * @Date: 2022-09-10
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-10
 * @FilePath: /algorithm/669_trim_bst/trim_bst.go
 */

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func trimBST(root *TreeNode, low, high int) *TreeNode {
	for root != nil && (root.Val < low || root.Val > high) {
		if root.Val < low {
			root = root.Right
		} else {
			root = root.Left
		}
	}
	if root == nil {
		return nil
	}
	for node := root; node.Left != nil; {
		if node.Left.Val < low {
			node.Left = node.Left.Right
		} else {
			node = node.Left
		}
	}
	for node := root; node.Right != nil; {
		if node.Right.Val > high {
			node.Right = node.Right.Left
		} else {
			node = node.Right
		}
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
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		root := &TreeNode{1, &TreeNode{0, nil, nil}, &TreeNode{2, nil, nil}}
		low := 1
		high := 2
		ans := &TreeNode{1, nil, &TreeNode{2, nil, nil}}
		assert(isSameTree(trimBST(root, low, high), ans))
	}

	{
		root := &TreeNode{3, &TreeNode{0, nil, &TreeNode{2, &TreeNode{1, nil, nil}, nil}}, &TreeNode{4, nil, nil}}
		low := 1
		high := 3
		ans := &TreeNode{3, &TreeNode{2, &TreeNode{1, nil, nil}, nil}, nil}
		assert(isSameTree(trimBST(root, low, high), ans))
	}
}
