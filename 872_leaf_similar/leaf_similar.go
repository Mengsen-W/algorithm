/*
 * @Date: 2021-05-10 08:45:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-10 09:30:43
 */

package main

import (
	"fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func leafSimilar(root1, root2 *TreeNode) bool {
	vals := []int{}
	var dfs func(*TreeNode)
	dfs = func(node *TreeNode) {
		if node == nil {
			return
		}
		if node.Left == nil && node.Right == nil {
			vals = append(vals, node.Val)
			return
		}
		dfs(node.Left)
		dfs(node.Right)
	}
	dfs(root1)
	vals1 := append([]int(nil), vals...)
	vals = []int{}
	dfs(root2)
	if len(vals) != len(vals1) {
		return false
	}
	for i, v := range vals1 {
		if v != vals[i] {
			return false
		}
	}
	return true
}

func assertTrue(a bool) {
	if !a {
		fmt.Printf("Not Pass")
	}
}

func main() {
	{
		root1 := &TreeNode{3, &TreeNode{5, &TreeNode{6, nil, nil}, &TreeNode{2, &TreeNode{7, nil, nil}, &TreeNode{4, nil, nil}}}, &TreeNode{1, &TreeNode{9, nil, nil}, &TreeNode{8, nil, nil}}}
		root2 := &TreeNode{3, &TreeNode{5, &TreeNode{6, nil, nil}, &TreeNode{7, nil, nil}}, &TreeNode{1, &TreeNode{4, nil, nil}, &TreeNode{2, &TreeNode{9, nil, nil}, &TreeNode{8, nil, nil}}}}
		assertTrue(leafSimilar(root1, root2))
	}
	{
		root1 := &TreeNode{1, nil, nil}
		root2 := &TreeNode{1, nil, nil}
		assertTrue(leafSimilar(root1, root2))
	}
	{
		root1 := &TreeNode{1, nil, nil}
		root2 := &TreeNode{2, nil, nil}
		assertTrue(!leafSimilar(root1, root2))
	}
	{
		root1 := &TreeNode{1, &TreeNode{2, nil, nil}, nil}
		root2 := &TreeNode{2, &TreeNode{2, nil, nil}, nil}
		assertTrue(leafSimilar(root1, root2))
	}
	{
		root1 := &TreeNode{1, &TreeNode{2, nil, nil}, &TreeNode{3, nil, nil}}
		root2 := &TreeNode{2, &TreeNode{3, nil, nil}, &TreeNode{2, nil, nil}}
		assertTrue(!leafSimilar(root1, root2))
	}
}
