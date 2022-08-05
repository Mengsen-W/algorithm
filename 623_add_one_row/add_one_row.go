/*
 * @Date: 2022-08-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-05
 * @FilePath: /algorithm/623_add_one_row/add_one_row.go
 */

package main

//  Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func addOneRow(root *TreeNode, val, depth int) *TreeNode {
	if depth == 1 {
		return &TreeNode{val, root, nil}
	}
	nodes := []*TreeNode{root}
	for i := 1; i < depth-1; i++ {
		tmp := nodes
		nodes = nil
		for _, node := range tmp {
			if node.Left != nil {
				nodes = append(nodes, node.Left)
			}
			if node.Right != nil {
				nodes = append(nodes, node.Right)
			}
		}
	}
	for _, node := range nodes {
		node.Left = &TreeNode{val, node.Left, nil}
		node.Right = &TreeNode{val, nil, node.Right}
	}
	return root
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
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		root := &TreeNode{4, &TreeNode{2, &TreeNode{3, nil, nil}, &TreeNode{1, nil, nil}}, &TreeNode{6, &TreeNode{5, nil, nil}, nil}}
		val := 1
		depth := 2
		ans := &TreeNode{4, &TreeNode{1, &TreeNode{2, &TreeNode{3, nil, nil}, &TreeNode{1, nil, nil}}, nil}, &TreeNode{1, nil, &TreeNode{6, &TreeNode{5, nil, nil}, nil}}}
		assert(isSameTree(addOneRow(root, val, depth), ans))
	}
	{
		root := &TreeNode{4, &TreeNode{2, &TreeNode{3, nil, nil}, &TreeNode{1, nil, nil}}, &TreeNode{6, &TreeNode{5, nil, nil}, nil}}
		val := 1
		depth := 2
		ans := &TreeNode{4, &TreeNode{1, &TreeNode{2, &TreeNode{3, nil, nil}, &TreeNode{1, nil, nil}}, nil}, &TreeNode{1, nil, &TreeNode{6, &TreeNode{5, nil, nil}, nil}}}
		assert(isSameTree(addOneRow(root, val, depth), ans))
	}

	{
		root := &TreeNode{4, &TreeNode{2, &TreeNode{3, nil, nil}, &TreeNode{1, nil, nil}}, nil}
		val := 1
		depth := 3
		ans := &TreeNode{4, &TreeNode{2, &TreeNode{1, &TreeNode{3, nil, nil}, nil}, &TreeNode{1, nil, &TreeNode{1, nil, nil}}}, nil}
		assert(isSameTree(addOneRow(root, val, depth), ans))
	}
}
