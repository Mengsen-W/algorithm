// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxLevelSum(root *TreeNode) int {
	ans, maxSum := 1, root.Val
	q := []*TreeNode{root}
	for level := 1; len(q) > 0; level++ {
		tmp := q
		q = nil
		sum := 0
		for _, node := range tmp {
			sum += node.Val
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
		if sum > maxSum {
			ans, maxSum = level, sum
		}
	}
	return ans
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  int
	}{
		{&TreeNode{1, &TreeNode{7, &TreeNode{7, nil, nil}, &TreeNode{-8, nil, nil}}, &TreeNode{0, nil, nil}}, 2},
		{&TreeNode{989, nil, &TreeNode{10250, &TreeNode{98693, nil, nil}, &TreeNode{-89388, nil, &TreeNode{-32127, nil, nil}}}}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxLevelSum(test.root), index)
	}
}
