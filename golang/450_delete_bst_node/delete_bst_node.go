/*
 * @Date: 2022-06-02 09:53:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-02 10:43:37
 * @FilePath: /algorithm/450_delete_bst_node/delete_bst_node.go
 */

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func deleteNode(root *TreeNode, key int) *TreeNode {
	var cur, curParent *TreeNode = root, nil
	for cur != nil && cur.Val != key {
		curParent = cur
		if cur.Val > key {
			cur = cur.Left
		} else {
			cur = cur.Right
		}
	}
	if cur == nil {
		return root
	}
	if cur.Left == nil && cur.Right == nil {
		cur = nil
	} else if cur.Right == nil {
		cur = cur.Left
	} else if cur.Left == nil {
		cur = cur.Right
	} else {
		successor, successorParent := cur.Right, cur
		for successor.Left != nil {
			successorParent = successor
			successor = successor.Left
		}
		if successorParent.Val == cur.Val {
			successorParent.Right = successor.Right
		} else {
			successorParent.Left = successor.Right
		}
		successor.Right = cur.Right
		successor.Left = cur.Left
		cur = successor
	}
	if curParent == nil {
		return cur
	}
	if curParent.Left != nil && curParent.Left.Val == key {
		curParent.Left = cur
	} else {
		curParent.Right = cur
	}
	return root
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(deleteNode(nil, 1) == nil)
	assert(deleteNode(&TreeNode{5, &TreeNode{3, &TreeNode{2, nil, nil}, &TreeNode{4, nil, nil}}, &TreeNode{6, nil, &TreeNode{7, nil, nil}}}, 3).Left.Val == 4)
	assert(deleteNode(&TreeNode{5, &TreeNode{2, nil, &TreeNode{4, nil, nil}}, &TreeNode{6, nil, &TreeNode{7, nil, nil}}}, 0).Val == 5)
	assert(deleteNode(nil, 0) == nil)
}
