/*
 * @Date: 2023-07-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-14
 * @FilePath: /algorithm/golang/979_distribute_coins/distribute_coins.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func distributeCoins(root *TreeNode) int {
	move := 0
	abs := func(a int) int {
		if a < 0 {
			a = -a
		}
		return a
	}
	var dfs func(root *TreeNode) int
	dfs = func(root *TreeNode) int {
		moveleft := 0
		moveright := 0
		if root == nil {
			return 0
		}
		if root.Left != nil {
			moveleft = dfs(root.Left)
		}
		if root.Right != nil {
			moveright = dfs(root.Right)
		}
		move += abs(moveleft) + abs(moveright)
		return moveleft + moveright + root.Val - 1
	}
	dfs(root)
	return move
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  int
	}{
		{&TreeNode{3, &TreeNode{0, nil, nil}, &TreeNode{0, nil, nil}}, 2},
		{&TreeNode{0, &TreeNode{3, nil, nil}, &TreeNode{0, nil, nil}}, 3},
		{&TreeNode{1, &TreeNode{0, nil, nil}, &TreeNode{2, nil, nil}}, 2},
		{&TreeNode{1, &TreeNode{0, nil, &TreeNode{3, nil, nil}}, &TreeNode{0, nil, nil}}, 4},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.root, item.ans)
	}
}
