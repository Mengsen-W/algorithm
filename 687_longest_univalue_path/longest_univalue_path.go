/*
 * @Date: 2022-09-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-02
 * @FilePath: /algorithm/687_longest_univalue_path/longest_univalue_path.go
 */

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func longestUnivaluePath(root *TreeNode) (ans int) {
	var dfs func(*TreeNode) int
	dfs = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		left := dfs(node.Left)
		right := dfs(node.Right)
		left1, right1 := 0, 0
		if node.Left != nil && node.Left.Val == node.Val {
			left1 = left + 1
		}
		if node.Right != nil && node.Right.Val == node.Val {
			right1 = right + 1
		}
		max := func(a, b int) int {
			if b > a {
				return b
			}
			return a
		}
		ans = max(ans, left1+right1)
		return max(left1, right1)
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
		root := &TreeNode{5, &TreeNode{4, &TreeNode{1, nil, nil}, &TreeNode{1, nil, nil}}, &TreeNode{5, nil, &TreeNode{5, nil, nil}}}
		ans := 2
		assert(longestUnivaluePath(root) == ans)
	}

	{
		root :=
			&TreeNode{1, &TreeNode{4, &TreeNode{4, nil, nil}, &TreeNode{4, nil, nil}}, &TreeNode{5, nil, &TreeNode{5, nil, nil}}}
		ans := 2
		assert(longestUnivaluePath(root) == ans)
	}
}
