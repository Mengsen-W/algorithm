/*
 * @Date: 2024-03-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-12
 * @FilePath: /algorithm/golang/1261_FindElements/FindElements.go
 */

// Package main ...
package main

import (
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type FindElements struct {
	Root *TreeNode
}

func Constructor(root *TreeNode) FindElements {
	dfs(root, 0)
	return FindElements{
		Root: root,
	}
}

func dfs(node *TreeNode, val int) {
	if node == nil {
		return
	}
	node.Val = val
	dfs(node.Left, val*2+1)
	dfs(node.Right, val*2+2)
}

func (this *FindElements) Find(target int) bool {
	target++
	k := 30 - bits.LeadingZeros32(uint32(target))
	node := this.Root
	for k >= 0 && node != nil {
		if (target & (1 << k)) == 0 {
			node = node.Left
		} else {
			node = node.Right
		}
		k--
	}
	return node != nil
}

func main() {
	findElements := &FindElements{&TreeNode{-1, nil, &TreeNode{-1, nil, nil}}}
	assert.Equal(&testing.T{}, false, findElements.Find(1))
	assert.Equal(&testing.T{}, true, findElements.Find(2))
}
