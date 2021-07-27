/*
 * @Date: 2021-07-27 17:38:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-27 18:44:26
 */

package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findSecondMinimumValue(root *TreeNode) int {
	ans := -1
	rootVal := root.Val
	var dfs func(*TreeNode)
	dfs = func(node *TreeNode) {
		if node == nil || ans != -1 && node.Val >= ans {
			return
		}
		if node.Val > rootVal {
			ans = node.Val
		}
		dfs(node.Left)
		dfs(node.Right)
	}
	dfs(root)
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		root := &TreeNode{Val: 2, Left: &TreeNode{2, nil, nil}, Right: &TreeNode{5, &TreeNode{5, nil, nil}, &TreeNode{7, nil, nil}}}
		assert(findSecondMinimumValue(root) == 5)
	}
	{
		root := &TreeNode{Val: 2, Left: &TreeNode{2, nil, nil}, Right: &TreeNode{2, nil, nil}}
		assert(findSecondMinimumValue(root) == -1)
	}
}
