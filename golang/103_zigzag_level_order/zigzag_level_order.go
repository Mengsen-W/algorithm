/*
 * @Date: 2024-02-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-16
 * @FilePath: /algorithm/golang/103_zigzag_level_order/zigzag_level_order.go
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

func zigzagLevelOrder(root *TreeNode) (ans [][]int) {
	if root == nil {
		return
	}
	queue := []*TreeNode{root}
	for level := 0; len(queue) > 0; level++ {
		vals := []int{}
		q := queue
		queue = nil
		for _, node := range q {
			vals = append(vals, node.Val)
			if node.Left != nil {
				queue = append(queue, node.Left)
			}
			if node.Right != nil {
				queue = append(queue, node.Right)
			}
		}
		// 本质上和层序遍历一样，我们只需要把奇数层的元素翻转即可
		if level%2 == 1 {
			for i, n := 0, len(vals); i < n/2; i++ {
				vals[i], vals[n-1-i] = vals[n-1-i], vals[i]
			}
		}
		ans = append(ans, vals)
	}
	return
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  [][]int
	}{
		{&TreeNode{3, &TreeNode{9, nil, nil}, &TreeNode{20, &TreeNode{15, nil, nil}, &TreeNode{7, nil, nil}}}, [][]int{{3}, {20, 9}, {15, 7}}},
		{&TreeNode{1, nil, nil}, [][]int{{1}}},
		{nil, nil},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, zigzagLevelOrder(test.root), index)
	}
}
