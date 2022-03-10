/*
 * @Date: 2022-03-10 02:37:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-10 03:16:21
 * @FilePath: /algorithm/589_ntree_preorder/ntree_preorder.go
 */

package main

import "reflect"

type Node struct {
	Val      int
	Children []*Node
}

func preorder(root *Node) (ans []int) {
	if root == nil {
		return
	}
	st := []*Node{root}
	for len(st) > 0 {
		node := st[len(st)-1]
		st = st[:len(st)-1]
		ans = append(ans, node.Val)
		for i := len(node.Children) - 1; i >= 0; i-- {
			st = append(st, node.Children[i])
		}
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		root := &Node{Val: 1}
		children := []*Node{{Val: 3}, {Val: 2}, {Val: 4}}
		children[0].Children = []*Node{{Val: 5}, {Val: 6}}
		root.Children = children
		assert(preorder(root), []int{1, 3, 5, 6, 2, 4})
	}

	{
		root := &Node{Val: 1}
		child := []*Node{{Val: 2}, {Val: 3}, {Val: 4}, {Val: 5}}
		child[1].Children = []*Node{{Val: 6}, {Val: 7}}
		child[2].Children = []*Node{{Val: 8}}
		child[3].Children = []*Node{{Val: 9}, {Val: 10}}
		child[1].Children[1].Children = []*Node{{Val: 11}}
		child[1].Children[1].Children[0].Children = []*Node{{Val: 14}}
		child[2].Children[0].Children = []*Node{{Val: 12}}
		child[3].Children[0].Children = []*Node{{Val: 13}}
		root.Children = child
		assert(preorder(root), []int{1, 2, 3, 6, 7, 11, 14, 4, 8, 12, 5, 9, 13, 10})
	}
}
