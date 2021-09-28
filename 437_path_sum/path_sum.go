/*
 * @Date: 2021-09-28 09:55:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-28 10:26:23
 * @FilePath: /algorithm/437_path_sum/path_sum.go
 * @Description: file content
 */

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func pathSum(root *TreeNode, targetSum int) (ans int) {
	preSum := map[int]int{0: 1}
	var dfs func(*TreeNode, int)
	dfs = func(node *TreeNode, curr int) {
		if node == nil {
			return
		}
		curr += node.Val
		ans += preSum[curr-targetSum]
		preSum[curr]++
		dfs(node.Left, curr)
		dfs(node.Right, curr)
		preSum[curr]--
		return
	}
	dfs(root, 0)
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		root := &TreeNode{
			10,
			&TreeNode{5, &TreeNode{3, &TreeNode{3, nil, nil}, &TreeNode{-2, nil, nil}},
				&TreeNode{2, nil, &TreeNode{1, nil, nil}}},
			&TreeNode{-3, nil, &TreeNode{11, nil, nil}}}
		assert(pathSum(root, 8) == 3)
	}
	{
		root := &TreeNode{
			5,
			&TreeNode{4, &TreeNode{11, &TreeNode{7, nil, nil}, &TreeNode{2, nil, nil}},
				nil},
			&TreeNode{8, &TreeNode{13, nil, nil},
				&TreeNode{4, &TreeNode{5, nil, nil}, &TreeNode{1, nil, nil}}}}
		assert(pathSum(root, 22) == 3)
	}
}
