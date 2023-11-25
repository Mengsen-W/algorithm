/*
 * @Date: 2023-11-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-25
 * @FilePath: /algorithm/golang/1457_pseudo_palindromic_paths/pseudo_palindromic_paths.go
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

func pseudoPalindromicPaths(root *TreeNode) int {
	counter := make([]int, 10)
	return dfs(root, counter)
}

func dfs(root *TreeNode, counter []int) int {
	if root == nil {
		return 0
	}
	counter[root.Val]++
	res := 0
	if root.Left == nil && root.Right == nil {
		if isPseudoPalindrome(counter) {
			res = 1
		}
	} else {
		res = dfs(root.Left, counter) + dfs(root.Right, counter)
	}
	counter[root.Val]--
	return res
}

func isPseudoPalindrome(counter []int) bool {
	odd := 0
	for _, value := range counter {
		if value%2 == 1 {
			odd++
		}
	}
	return odd <= 1
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  int
	}{
		{
			&TreeNode{Val: 2, Left: &TreeNode{Val: 3, Left: &TreeNode{Val: 3, Left: nil, Right: nil}, Right: &TreeNode{Val: 1}}, Right: &TreeNode{Val: 1, Left: nil, Right: &TreeNode{Val: 1, Left: nil, Right: nil}}},
			2,
		},
		{
			&TreeNode{Val: 2, Left: &TreeNode{Val: 1, Left: &TreeNode{Val: 1}, Right: &TreeNode{Val: 3, Left: nil, Right: &TreeNode{Val: 3}}}, Right: &TreeNode{Val: 1}},
			1,
		},
		{
			&TreeNode{Val: 1, Left: nil, Right: nil},
			1,
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, pseudoPalindromicPaths(test.root), index)
	}
}
