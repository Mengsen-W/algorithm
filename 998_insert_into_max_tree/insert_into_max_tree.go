/*
 * @Date: 2022-08-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-30
 * @FilePath: /algorithm/998_insert_into_max_tree/insert_into_max_tree.go
 */

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func insertIntoMaxTree(root *TreeNode, val int) *TreeNode {
	var parent *TreeNode
	for cur := root; cur != nil; cur = cur.Right {
		if val > cur.Val {
			if parent == nil {
				return &TreeNode{val, root, nil}
			}
			parent.Right = &TreeNode{val, cur, nil}
			return root
		}
		parent = cur
	}
	parent.Right = &TreeNode{Val: val}
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
		root := &TreeNode{4, &TreeNode{1, nil, nil}, &TreeNode{3, &TreeNode{2, nil, nil}, nil}}
		val := 5
		ans := &TreeNode{5, &TreeNode{4, &TreeNode{1, nil, nil}, &TreeNode{3, &TreeNode{2, nil, nil}, nil}}, nil}
		assert(isSameTree(insertIntoMaxTree(root, val), ans))
	}

	{
		root := &TreeNode{5, &TreeNode{2, nil, &TreeNode{1, nil, nil}}, &TreeNode{4, nil, nil}}
		val := 3
		ans := &TreeNode{5, &TreeNode{2, nil, &TreeNode{1, nil, nil}}, &TreeNode{4, nil, &TreeNode{3, nil, nil}}}
		assert(isSameTree(insertIntoMaxTree(root, val), ans))
	}

	{
		root := &TreeNode{5, &TreeNode{2, nil, &TreeNode{1, nil, nil}}, &TreeNode{3, nil, nil}}
		val := 4
		ans := &TreeNode{5, &TreeNode{2, nil, &TreeNode{1, nil, nil}}, &TreeNode{4, &TreeNode{3, nil, nil}, nil}}
		assert(isSameTree(insertIntoMaxTree(root, val), ans))
	}
}
