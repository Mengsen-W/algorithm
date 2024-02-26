/*
 * @Date: 2024-02-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-25
 * @FilePath: /algorithm/golang/235_lowest_common_ancestor/lowest_common_ancestor.go
 */

// Package main ...
package main

// TreeNode Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func getPath(root, target *TreeNode) (path []*TreeNode) {
	node := root
	for node != target {
		path = append(path, node)
		if target.Val < node.Val {
			node = node.Left
		} else {
			node = node.Right
		}
	}
	path = append(path, node)
	return
}

func lowestCommonAncestor(root, p, q *TreeNode) (ancestor *TreeNode) {
	pathP := getPath(root, p)
	pathQ := getPath(root, q)
	for i := 0; i < len(pathP) && i < len(pathQ) && pathP[i] == pathQ[i]; i++ {
		ancestor = pathP[i]
	}
	return
}

func main() {}
