/*
 * @Date: 2022-07-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-21
 * @FilePath: /algorithm/814_prune_tree/prune_tree.go
 */

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func pruneTree(root *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}
	root.Left = pruneTree(root.Left)
	root.Right = pruneTree(root.Right)
	if root.Left == nil && root.Right == nil && root.Val == 0 {
		return nil
	}
	return root
}

func main() {
	pruneTree(&TreeNode{Val: 1, Left: nil, Right: &TreeNode{Val: 0, Left: &TreeNode{Val: 0}, Right: &TreeNode{Val: 1}}})
	pruneTree(&TreeNode{Val: 1, Left: &TreeNode{Val: 0, Left: &TreeNode{Val: 0}, Right: &TreeNode{Val: 0}}, Right: &TreeNode{Val: 0, Left: &TreeNode{Val: 0}, Right: &TreeNode{Val: 1}}})
	pruneTree(&TreeNode{Val: 1, Left: &TreeNode{Val: 1, Left: &TreeNode{Val: 1, Left: &TreeNode{Val: 1}}, Right: &TreeNode{Val: 1}}, Right: &TreeNode{Val: 0, Left: &TreeNode{Val: 0}, Right: &TreeNode{Val: 1}}})
}
