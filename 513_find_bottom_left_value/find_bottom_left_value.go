/*
 * @Author: Mengsen Wang mengsen_wang@163.com
 * @Date: 2022-06-22
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-22
 * @FilePath: /algorithm/513_find_bottom_left_value/find_bottom_left_value.go
 */

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findBottomLeftValue(root *TreeNode) (ans int) {
	q := []*TreeNode{root}
	for len(q) > 0 {
		node := q[0]
		q = q[1:]
		if node.Right != nil {
			q = append(q, node.Right)
		}
		if node.Left != nil {
			q = append(q, node.Left)
		}
		ans = node.Val
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
		root := &TreeNode{2, &TreeNode{1, nil, nil}, &TreeNode{3, nil, nil}}
		assert(findBottomLeftValue(root) == 1)
	}

	{
		root := &TreeNode{1, &TreeNode{2, &TreeNode{4, nil, nil}, nil}, &TreeNode{3, &TreeNode{5, &TreeNode{7, nil, nil}, nil}, &TreeNode{6, nil, nil}}}
		assert(findBottomLeftValue(root) == 7)
	}
}
