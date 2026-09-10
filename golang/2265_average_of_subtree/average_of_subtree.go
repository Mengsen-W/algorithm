// Package main ...
package main

import "fmt"

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func averageOfSubtree(root *TreeNode) int {
	ans := 0
	var dfs func(*TreeNode) (int, int)
	dfs = func(node *TreeNode) (int, int) {
		if node == nil {
			return 0, 0
		}
		leftSum, leftSize := dfs(node.Left)
		rightSum, rightSize := dfs(node.Right)
		Size := leftSize + rightSize + 1
		Sum := leftSum + rightSum + node.Val
		if Size > 0 && Sum/Size == node.Val {
			ans++
		}
		return Sum, Size
	}
	dfs(root)
	return ans
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  int
	}{
		{
			&TreeNode{4, &TreeNode{8, &TreeNode{0, nil, nil}, &TreeNode{1, nil, nil}}, &TreeNode{5, nil, &TreeNode{6, nil, nil}}},
			5,
		},
		{
			&TreeNode{1,nil,nil},
			1,
		},
	}

	for index, test := range tests {
		if averageOfSubtree(test.root) != test.ans {
			fmt.Println(index)
		}
	}
}
