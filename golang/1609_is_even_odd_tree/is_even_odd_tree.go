/*
 * @Date: 2021-12-25 01:39:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-25 06:38:23
 */

package main

import (
	"math"
)

//  Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isEvenOddTree(root *TreeNode) bool {
	q := []*TreeNode{root}
	for level := 0; len(q) > 0; level++ {
		prev := 0
		if level%2 == 1 {
			prev = math.MaxInt32
		}
		size := len(q)
		for _, node := range q {
			val := node.Val
			if val%2 == level%2 || level%2 == 0 && val <= prev || level%2 == 1 && val >= prev {
				return false
			}
			prev = val
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
		q = q[size:]
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		root := &TreeNode{1, &TreeNode{10, &TreeNode{3, &TreeNode{12, nil, nil}, &TreeNode{8, nil, nil}}, nil}, &TreeNode{4, &TreeNode{7, &TreeNode{6, nil, nil}, nil}, &TreeNode{9, nil, &TreeNode{2, nil, nil}}}}
		assert(isEvenOddTree(root))
	}

	{
		root := &TreeNode{5, &TreeNode{4, &TreeNode{3, nil, nil}, &TreeNode{3, nil, nil}}, &TreeNode{2, &TreeNode{7, nil, nil}, nil}}
		assert(!isEvenOddTree(root))
	}

	{
		root := &TreeNode{5, &TreeNode{9, &TreeNode{3, nil, nil}, &TreeNode{5, nil, nil}}, &TreeNode{1, &TreeNode{7, nil, nil}, nil}}
		assert(!isEvenOddTree(root))
	}

	{
		root := &TreeNode{1, nil, nil}
		assert(isEvenOddTree(root))
	}

	{
		root := &TreeNode{11, &TreeNode{8, &TreeNode{1, &TreeNode{30, &TreeNode{17, nil, nil}, nil}, &TreeNode{20, nil, nil}}, &TreeNode{3, &TreeNode{18, nil, nil}, &TreeNode{16, nil, nil}}}, &TreeNode{6, &TreeNode{9, &TreeNode{12, nil, nil}, &TreeNode{10, nil, nil}}, &TreeNode{11, &TreeNode{4, nil, nil}, &TreeNode{2, nil, nil}}}}
		assert(isEvenOddTree(root))
	}
}
