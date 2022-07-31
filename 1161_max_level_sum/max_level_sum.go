/*
 * @Date: 2022-07-31
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-31
 * @FilePath: /algorithm/1161_max_level_sum/max_level_sum.go
 */

package main

// Definition for a binary tree node.
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
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		root := &TreeNode{1, &TreeNode{7, &TreeNode{7, nil, nil}, &TreeNode{-8, nil, nil}}, &TreeNode{0, nil, nil}}
		assert(maxLevelSum(root) == 2)
	}
	{
		root := &TreeNode{989, nil, &TreeNode{10250, &TreeNode{98693, nil, nil},
			&TreeNode{-89388, nil, &TreeNode{-32127, nil, nil}}}}
		assert(maxLevelSum(root) == 2)
	}
}
