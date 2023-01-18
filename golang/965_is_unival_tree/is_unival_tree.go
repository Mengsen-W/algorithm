/*
 * @Date: 2022-05-24 19:50:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-24 20:05:04
 * @FilePath: /algorithm/965_is_unival_tree/is_unival_tree.go
 */

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isUnivalTree(root *TreeNode) bool {
	return root == nil || (root.Left == nil || root.Val == root.Left.Val && isUnivalTree(root.Left)) &&
		(root.Right == nil || root.Val == root.Right.Val && isUnivalTree(root.Right))
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(isUnivalTree(&TreeNode{Val: 1, Left: &TreeNode{Val: 1, Left: &TreeNode{Val: 1}, Right: &TreeNode{Val: 1}}, Right: &TreeNode{Val: 1, Right: &TreeNode{Val: 1}}}) == true)
	assert(isUnivalTree(&TreeNode{Val: 2, Left: &TreeNode{Val: 2, Left: &TreeNode{Val: 5}, Right: &TreeNode{Val: 2}}, Right: &TreeNode{Val: 2}}) == false)
}
