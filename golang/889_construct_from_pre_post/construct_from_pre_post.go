/*
 * @Date: 2024-02-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-22
 * @FilePath: /algorithm/golang/889_construct_from_pre_post/construct_from_pre_post.go
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

func constructFromPrePost(preorder []int, postorder []int) *TreeNode {
	postMap := map[int]int{}
	for i, v := range postorder {
		postMap[v] = i
	}
	var dfs func(int, int, int, int) *TreeNode
	dfs = func(preLeft, preRight, postLeft, postRight int) *TreeNode {
		if preLeft > preRight {
			return nil
		}
		leftCount := 0
		if preLeft < preRight {
			leftCount = postMap[preorder[preLeft+1]] - postLeft + 1
		}
		return &TreeNode{
			Val:   preorder[preLeft],
			Left:  dfs(preLeft+1, preLeft+leftCount, postLeft, postLeft+leftCount-1),
			Right: dfs(preLeft+leftCount+1, preRight, postLeft+leftCount, postRight-1),
		}
	}
	return dfs(0, len(preorder)-1, 0, len(postorder)-1)
}

func isSameTree(p, q *TreeNode) bool {
	if p == nil || q == nil {
		return p == q
	}

	if p.Val != q.Val {
		return false
	}

	return isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}

func main() {
	tests := []struct {
		preorder, postorder []int
		ans                 *TreeNode
	}{
		{
			[]int{1, 2, 4, 5, 3, 6, 7},
			[]int{4, 5, 2, 6, 7, 3, 1},
			&TreeNode{
				1, &TreeNode{2, &TreeNode{4, nil, nil}, &TreeNode{5, nil, nil}},
				&TreeNode{3, &TreeNode{6, nil, nil}, &TreeNode{7, nil, nil}},
			},
		},
		{
			[]int{1},
			[]int{1},
			&TreeNode{1, nil, nil},
		},
	}

	for index, test := range tests {
		assert.True(&testing.T{}, isSameTree(constructFromPrePost(test.preorder, test.postorder), test.ans), index)
	}
}
