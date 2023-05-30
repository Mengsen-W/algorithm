/*
 * @Date: 2023-05-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-30
 * @FilePath: /algorithm/golang/1110_del_nodes/del_nodes.go
 */

// Package main ...
package main

import (
	"reflect"
)

// TreeNode Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func delNodes(root *TreeNode, toDelete []int) []*TreeNode {
	toDeleteSet := make(map[int]bool)
	for _, val := range toDelete {
		toDeleteSet[val] = true
	}
	var roots []*TreeNode
	dfs(root, true, toDeleteSet, &roots)
	return roots
}

func dfs(node *TreeNode, isRoot bool, toDeleteSet map[int]bool, roots *[]*TreeNode) *TreeNode {
	if node == nil {
		return nil
	}
	deleted := toDeleteSet[node.Val]
	node.Left = dfs(node.Left, deleted, toDeleteSet, roots)
	node.Right = dfs(node.Right, deleted, toDeleteSet, roots)
	if deleted {
		return nil
	} else if isRoot {
		*roots = append(*roots, node)
	}
	return node
}

func main() {
	assert := func(a, b []*TreeNode) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		root := &TreeNode{1, &TreeNode{2, &TreeNode{4, nil, nil}, &TreeNode{5, nil, nil}}, &TreeNode{3, &TreeNode{6, nil, nil}, &TreeNode{7, nil, nil}}}
		toDelete := []int{3, 5}
		ans := []*TreeNode{
			{6, nil, nil},
			{1, &TreeNode{2, nil, &TreeNode{4, nil, nil}}, nil},
			{7, nil, nil},
		}
		assert(delNodes(root, toDelete), ans)
	}

	{
		root := &TreeNode{1, &TreeNode{2, nil, &TreeNode{3, nil, nil}}, &TreeNode{4, nil, nil}}
		to_delete := []int{3}
		ans := []*TreeNode{
			{1, &TreeNode{2, &TreeNode{4, nil, nil}, nil}, nil},
		}
		assert(delNodes(root, to_delete), ans)
	}
}
