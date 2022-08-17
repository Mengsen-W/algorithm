/*
 * @Date: 2022-08-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-17
 * @FilePath: /algorithm/1302_deepest_leaves_sum/deepest_leaves_sum.go
 */

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func deepestLeavesSum(root *TreeNode) (sum int) {
	q := []*TreeNode{root}
	for len(q) > 0 {
		sum = 0
		size := len(q)
		for i := 0; i < size; i++ {
			node := q[0]
			q = q[1:]
			sum += node.Val
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		root := &TreeNode{1, &TreeNode{2, &TreeNode{4, &TreeNode{7, nil, nil}, nil}, &TreeNode{5, nil, nil}}, &TreeNode{3, nil, &TreeNode{6, nil, &TreeNode{8, nil, nil}}}}
		assert(deepestLeavesSum(root) == 15)
	}

	{
		root := &TreeNode{6, &TreeNode{7, &TreeNode{2, &TreeNode{9, nil, nil}, nil}, &TreeNode{7, &TreeNode{1, nil, nil}, &TreeNode{4, nil, nil}}}, &TreeNode{8, &TreeNode{1, nil, nil}, &TreeNode{3, nil, &TreeNode{5, nil, nil}}}}
		assert(deepestLeavesSum(root) == 19)
	}
}
