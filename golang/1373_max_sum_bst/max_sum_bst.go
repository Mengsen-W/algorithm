/*
 * @Date: 2023-05-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-20
 * @FilePath: /algorithm/golang/1373_max_sum_bst/max_sum_bst.go
 */

// Package main ...
package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

const INF = 0x3f3f3f3f

type SubTree struct {
	IsBST  bool
	MinVal int
	MaxVal int
	SumVal int
}

var res int

func maxSumBST(root *TreeNode) int {
	res = 0
	dfs(root)
	return res
}

func max(a int, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a int, b int) int {
	if a < b {
		return a
	}
	return b
}

func dfs(root *TreeNode) *SubTree {
	if root == nil {
		return &SubTree{true, INF, -INF, 0}
	}
	left := dfs(root.Left)
	right := dfs(root.Right)

	if left.IsBST && right.IsBST && root.Val > left.MaxVal && root.Val < right.MinVal {
		sum := root.Val + left.SumVal + right.SumVal
		res = max(res, sum)
		return &SubTree{true, min(left.MinVal, root.Val), max(root.Val, right.MaxVal), sum}
	} else {
		return &SubTree{false, 0, 0, 0}
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		root := &TreeNode{1, &TreeNode{4, &TreeNode{2, nil, nil}, &TreeNode{4, nil, nil}}, &TreeNode{3, &TreeNode{2, nil,
			nil}, &TreeNode{5, &TreeNode{4, nil, nil}, &TreeNode{6, nil, nil}}}}
		ans := 20
		assert(maxSumBST(root) == ans)
	}

	{
		root := &TreeNode{4, &TreeNode{3, &TreeNode{1, nil, nil}, &TreeNode{2, nil, nil}}, nil}
		ans := 2
		assert(maxSumBST(root) == ans)
	}

	{
		root := &TreeNode{-4, &TreeNode{-2, nil, nil}, &TreeNode{-5, nil, nil}}
		ans := 0
		assert(maxSumBST(root) == ans)
	}

	{
		root := &TreeNode{2, &TreeNode{1, nil, nil}, &TreeNode{3, nil, nil}}
		ans := 6
		assert(maxSumBST(root) == ans)
	}

	{
		root := &TreeNode{5, &TreeNode{4, &TreeNode{3, nil, nil}, nil}, &TreeNode{8, &TreeNode{6, nil, nil}, &TreeNode{3,
			nil, nil}}}
		ans := 7
		assert(maxSumBST(root) == ans)
	}
}
