/*
 * @Date: 2024-02-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-15
 * @FilePath: /algorithm/golang/107_level_order_bottom/level_order_bottom.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// TreeNode Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func levelOrderBottom(root *TreeNode) [][]int {
	levelOrder := [][]int{}
	if root == nil {
		return levelOrder
	}
	queue := []*TreeNode{}
	queue = append(queue, root)
	for len(queue) > 0 {
		level := []int{}
		size := len(queue)
		for i := 0; i < size; i++ {
			node := queue[0]
			queue = queue[1:]
			level = append(level, node.Val)
			if node.Left != nil {
				queue = append(queue, node.Left)
			}
			if node.Right != nil {
				queue = append(queue, node.Right)
			}
		}
		levelOrder = append(levelOrder, level)
	}
	for i := 0; i < len(levelOrder)/2; i++ {
		levelOrder[i], levelOrder[len(levelOrder)-1-i] = levelOrder[len(levelOrder)-1-i], levelOrder[i]
	}
	return levelOrder
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  [][]int
	}{
		{&TreeNode{3, &TreeNode{9, nil, nil}, &TreeNode{20, &TreeNode{15, nil, nil}, &TreeNode{7, nil, nil}}}, [][]int{{15, 7}, {9, 20}, {3}}},
		{&TreeNode{1, nil, nil}, [][]int{{1}}},
		{nil, [][]int{}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, levelOrderBottom(test.root), index)
	}
}
