/*
 * @Date: 2024-02-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-23
 * @FilePath: /algorithm/golang/2583_kth_largest_level_sum/kth_largest_level_sum.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

// TreeNode Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func kthLargestLevelSum(root *TreeNode, k int) int64 {
	q := []*TreeNode{root}
	var levelSumArray []int64
	for len(q) > 0 {
		levelSum, size := int64(0), len(q)
		for i := 0; i < size; i++ {
			node := q[0]
			q = q[1:]
			levelSum += int64(node.Val)
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
		levelSumArray = append(levelSumArray, levelSum)
	}
	if len(levelSumArray) < k {
		return -1
	}
	sort.Slice(levelSumArray, func(i, j int) bool {
		return levelSumArray[i] < levelSumArray[j]
	})
	return levelSumArray[len(levelSumArray)-k]
}

func main() {
	tests := []struct {
		root *TreeNode
		k    int
		ans  int64
	}{
		{
			&TreeNode{
				5, &TreeNode{8, &TreeNode{2, &TreeNode{4, nil, nil}, &TreeNode{6, nil, nil}}, &TreeNode{1, nil, nil}},
				&TreeNode{9, &TreeNode{3, nil, nil}, &TreeNode{7, nil, nil}},
			},
			2,
			13,
		},
		{
			&TreeNode{1, &TreeNode{2, &TreeNode{3, nil, nil}, nil}, nil},
			1,
			3,
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, kthLargestLevelSum(test.root, test.k), index)
	}
}
