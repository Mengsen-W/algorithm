/*
 * @Date: 2022-08-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-20
 * @FilePath: /algorithm/654_construct_maximum_binary_tree/construct_maximum_binary_tree.go
 */

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func constructMaximumBinaryTree(nums []int) *TreeNode {
	tree := make([]*TreeNode, len(nums))
	stk := []int{}
	for i, num := range nums {
		tree[i] = &TreeNode{Val: num}
		for len(stk) > 0 && num > nums[stk[len(stk)-1]] {
			tree[i].Left = tree[stk[len(stk)-1]]
			stk = stk[:len(stk)-1]
		}
		if len(stk) > 0 {
			tree[stk[len(stk)-1]].Right = tree[i]
		}
		stk = append(stk, i)
	}
	return tree[stk[0]]
}

func isSameTree(p *TreeNode, q *TreeNode) bool {
	if p == nil && q == nil {
		return true
	}
	if p == nil || q == nil {
		return false
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
		nums := []int{3, 2, 1, 6, 0, 5}
		ans := &TreeNode{6, &TreeNode{3, nil, &TreeNode{2, nil, &TreeNode{1, nil, nil}}}, &TreeNode{5, &TreeNode{0, nil, nil}, nil}}
		assert(isSameTree(constructMaximumBinaryTree(nums), ans))
	}
	{
		nums := []int{3, 2, 1}
		ans := &TreeNode{3, nil, &TreeNode{2, nil, &TreeNode{1, nil, nil}}}
		assert(isSameTree(constructMaximumBinaryTree(nums), ans))
	}
}
