/*
 * @Date: 2022-03-12 02:01:14
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-19
 * @FilePath: /algorithm/golang/590_ntree_postorder/ntree_postorder.go
 */

package main

import "reflect"

type Node struct {
	Val      int
	Children []*Node
}

func postorder(root *Node) (ans []int) {
	if root == nil {
		return
	}
	st := []*Node{root}
	for len(st) > 0 {
		node := st[len(st)-1]
		st = st[:len(st)-1]
		ans = append(ans, node.Val)
		st = append(st, node.Children...)
	}
	for i, n := 0, len(ans); i < n/2; i++ {
		ans[i], ans[n-1-i] = ans[n-1-i], ans[i]
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
		assert(postorder(root), []int{5, 6, 3, 2, 4, 1})
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
		assert(postorder(root), []int{2, 6, 14, 11, 7, 3, 12, 8, 4, 13, 9, 10, 5, 1})
	}
}
