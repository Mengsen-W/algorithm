/*
 * @Date: 2024-02-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-24
 * @FilePath: /algorithm/golang/2476_closest_nodes/closest_nodes.go
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

func closestNodes(root *TreeNode, queries []int) [][]int {
	arr := []int{}
	var dfs func(*TreeNode)
	dfs = func(root *TreeNode) {
		if root == nil {
			return
		}
		dfs(root.Left)
		arr = append(arr, root.Val)
		dfs(root.Right)
	}

	dfs(root)
	res := make([][]int, len(queries))
	for i, val := range queries {
		maxVal, minVal := -1, -1
		index := sort.SearchInts(arr, val)
		if index < len(arr) {
			maxVal = arr[index]
			if arr[index] == val {
				minVal = arr[index]
				res[i] = []int{minVal, maxVal}
				continue
			}
		}
		if index != 0 {
			minVal = arr[index-1]
		}
		res[i] = []int{minVal, maxVal}
	}
	return res
}

func main() {
	tests := []struct {
		root    *TreeNode
		queries []int
		ans     [][]int
	}{
		{
			&TreeNode{
				6, &TreeNode{2, &TreeNode{1, nil, nil}, &TreeNode{4, nil, nil}},
				&TreeNode{13, &TreeNode{9, nil, nil}, &TreeNode{15, &TreeNode{14, nil, nil}, nil}},
			},
			[]int{2, 5, 16},
			[][]int{{2, 2}, {4, 6}, {15, -1}},
		},
		{&TreeNode{4, nil, &TreeNode{9, nil, nil}}, []int{3}, [][]int{{-1, 4}}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, closestNodes(test.root, test.queries), index)
	}
}
