/*
 * @Date: 2023-02-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-06
 * @FilePath: /algorithm/golang/2331_evaluate_tree/evaluate_tree.go
 */

package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func evaluateTree(root *TreeNode) bool {
	if root.Left == nil {
		return root.Val == 1
	}
	if root.Val == 2 {
		return evaluateTree(root.Left) || evaluateTree(root.Right)
	}
	return evaluateTree(root.Left) && evaluateTree(root.Right)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		root := &TreeNode{2, &TreeNode{1, nil, nil}, &TreeNode{3, &TreeNode{0, nil, nil}, &TreeNode{1, nil, nil}}}
		ans := true
		assert(evaluateTree(root) == ans)
	}

	{
		root := &TreeNode{0, nil, nil}
		ans := false
		assert(evaluateTree(root) == ans)
	}
}
