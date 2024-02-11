/*
 * @Date: 2024-02-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-09
 * @FilePath: /algorithm/golang/236_lowest_common_ancestor/lowest_common_ancestor.go
 */

// Package main ...
package main

// TreeNode Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	if root == nil || root == p || root == q {
		return root
	}
	left := lowestCommonAncestor(root.Left, p, q)
	right := lowestCommonAncestor(root.Right, p, q)
	if left != nil && right != nil {
		return root
	}
	if left != nil {
		return left
	}
	return right
}
