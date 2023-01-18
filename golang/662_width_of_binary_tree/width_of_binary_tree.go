/*
 * @Date: 2022-08-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-27
 * @FilePath: /algorithm/662_width_of_binary_tree/width_of_binary_tree.go
 */

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func widthOfBinaryTree(root *TreeNode) int {
	type pair struct {
		node  *TreeNode
		index int
	}
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	ans := 1
	q := []pair{{root, 1}}
	for q != nil {
		ans = max(ans, q[len(q)-1].index-q[0].index+1)
		tmp := q
		q = nil
		for _, p := range tmp {
			if p.node.Left != nil {
				q = append(q, pair{p.node.Left, p.index * 2})
			}
			if p.node.Right != nil {
				q = append(q, pair{p.node.Right, p.index*2 + 1})
			}
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
		root := &TreeNode{1, &TreeNode{3, &TreeNode{5, nil, nil}, &TreeNode{3, nil, nil}}, &TreeNode{2, nil, &TreeNode{9, nil, nil}}}
		ans := 4
		assert(widthOfBinaryTree(root) == ans)
	}

	{
		root := &TreeNode{1, &TreeNode{3, &TreeNode{5, &TreeNode{6, nil, nil}, nil}, nil},
			&TreeNode{2, nil, &TreeNode{9, &TreeNode{7, nil, nil}, nil}}}
		ans := 7
		assert(widthOfBinaryTree(root) == ans)
	}

	{
		root := &TreeNode{1, &TreeNode{3, &TreeNode{5, nil, nil}, nil}, &TreeNode{2, nil, nil}}
		ans := 2
		assert(widthOfBinaryTree(root) == ans)
	}
}
