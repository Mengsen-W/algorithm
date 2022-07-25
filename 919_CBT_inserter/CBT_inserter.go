/*
 * @Date: 2022-07-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-25
 * @FilePath: /algorithm/919_CBT_inserter/CBT_inserter.go
 */
package main

import "math/bits"

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type CBTInserter struct {
	root *TreeNode
	cnt  int
}

func Constructor(root *TreeNode) CBTInserter {
	q := []*TreeNode{root}
	cnt := 0
	for len(q) > 0 {
		cnt++
		node := q[0]
		q = q[1:]
		if node.Left != nil {
			q = append(q, node.Left)
		}
		if node.Right != nil {
			q = append(q, node.Right)
		}
	}
	return CBTInserter{root, cnt}
}

func (c *CBTInserter) Insert(val int) int {
	c.cnt++
	child := &TreeNode{Val: val}
	node := c.root
	for i := bits.Len(uint(c.cnt)) - 2; i > 0; i-- {
		if c.cnt>>i&1 == 0 {
			node = node.Left
		} else {
			node = node.Right
		}
	}
	if c.cnt&1 == 0 {
		node.Left = child
	} else {
		node.Right = child
	}
	return node.Val
}

func (c *CBTInserter) Get_root() *TreeNode {
	return c.root
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}
	root := &TreeNode{1, &TreeNode{2, nil, nil}, nil}
	c := Constructor(root)
	assert(c.Insert(3), 1)
	assert(c.Insert(4), 2)
	assert(c.Get_root().Val, root.Val)
}
