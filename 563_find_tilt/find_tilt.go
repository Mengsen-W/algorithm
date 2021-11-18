/*
 * @Date: 2021-11-18 00:18:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-18 00:33:41
 */

package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findTilt(root *TreeNode) (ans int) {
	var dfs func(*TreeNode) int
	dfs = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		sumLeft := dfs(node.Left)
		sumRight := dfs(node.Right)
		ans += abs(sumLeft - sumRight)
		return sumLeft + sumRight + node.Val
	}
	dfs(root)
	return
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		root := &TreeNode{1, &TreeNode{2, nil, nil}, &TreeNode{3, nil, nil}}
		assert(findTilt(root) == 1)

	}
	{
		root := &TreeNode{4, &TreeNode{2, &TreeNode{3, nil, nil}, &TreeNode{5, nil, nil}}, &TreeNode{9, nil, &TreeNode{7, nil, nil}}}
		assert(findTilt(root) == 15)
	}
	{
		root := &TreeNode{21, &TreeNode{7, &TreeNode{1, &TreeNode{3, nil, nil}, &TreeNode{3, nil, nil}}, &TreeNode{1, nil, nil}}, &TreeNode{14, &TreeNode{2, nil, nil}, &TreeNode{2, nil, nil}}}
		assert(findTilt(root) == 9)
	}
}
