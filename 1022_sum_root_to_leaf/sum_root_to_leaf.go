/*
 * @Date: 2022-05-31 09:44:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-31 09:56:16
 * @FilePath: /algorithm/1022_sum_root_to_leaf/sum_root_to_leaf.go
 */

package main

//  Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sumRootToLeaf(root *TreeNode) (ans int) {
	val, st := 0, []*TreeNode{}
	var pre *TreeNode
	for root != nil || len(st) > 0 {
		for root != nil {
			val = val<<1 | root.Val
			st = append(st, root)
			root = root.Left
		}
		root = st[len(st)-1]
		if root.Right == nil || root.Right == pre {
			if root.Left == nil && root.Right == nil {
				ans += val
			}
			val >>= 1
			st = st[:len(st)-1]
			pre = root
			root = nil
		} else {
			root = root.Right
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(sumRootToLeaf(&TreeNode{1, &TreeNode{0, &TreeNode{0, nil, nil}, &TreeNode{1, nil, nil}}, &TreeNode{1, &TreeNode{0, nil, nil}, &TreeNode{1, nil, nil}}}) == 22)
	assert(sumRootToLeaf(&TreeNode{0, nil, nil}) == 0)
}
