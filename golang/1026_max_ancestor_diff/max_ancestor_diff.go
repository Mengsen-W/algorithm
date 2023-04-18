/*
 * @Date: 2023-04-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-18
 * @FilePath: /algorithm/golang/1026_max_ancestor_diff/max_ancestor_diff.go
 */

// Package main ...
package main

// TreeNode Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxAncestorDiff(root *TreeNode) (ans int) {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	var dfs func(*TreeNode, int, int)
	dfs = func(root *TreeNode, mi, mx int) {
		if root == nil {
			return
		}
		ans = max(ans, max(abs(mi-root.Val), abs(mx-root.Val)))
		mi = min(mi, root.Val)
		mx = max(mx, root.Val)
		dfs(root.Left, mi, mx)
		dfs(root.Right, mi, mx)
	}
	dfs(root, root.Val, root.Val)
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		root := &TreeNode{8, &TreeNode{3, &TreeNode{1, nil, nil}, &TreeNode{6, &TreeNode{4, nil, nil}, &TreeNode{7, nil, nil}}}, &TreeNode{10, nil, &TreeNode{14, &TreeNode{13, nil, nil}, nil}}}
		ans := 7
		assert(maxAncestorDiff(root) == ans)
	}

	{
		root := &TreeNode{1, nil, &TreeNode{2, nil, &TreeNode{0, &TreeNode{3, nil, nil}, nil}}}
		ans := 3
		assert(maxAncestorDiff(root) == ans)
	}
}
