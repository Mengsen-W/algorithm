// Package main ...
package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isBalanced(root *TreeNode) bool {
	return height(root) >= 0
}

func height(root *TreeNode) int {
	if root == nil {
		return 0
	}
	leftHeight := height(root.Left)
	rightHeight := height(root.Right)
	if leftHeight == -1 || rightHeight == -1 || abs(leftHeight-rightHeight) > 1 {
		return -1
	}
	return max(leftHeight, rightHeight) + 1
}

func max(x, y int) int {
	if x > y {
		return x
	}
	return y
}

func abs(x int) int {
	if x < 0 {
		return -1 * x
	}
	return x
}

func main() {
	tests := []struct {
		root *TreeNode
		ans  bool
	}{
		{
			&TreeNode{3, &TreeNode{9, nil, nil}, &TreeNode{20, &TreeNode{15, nil, nil}, &TreeNode{7, nil, nil}}},
			true,
		},
		{
			&TreeNode{1, &TreeNode{2, &TreeNode{3, &TreeNode{4, nil, nil}, &TreeNode{4, nil, nil}}, &TreeNode{3, nil, nil}}, &TreeNode{2, nil, nil}},
			false,
		},
		{
			nil,
			true,
		},
	}

	for index, text := range tests {
		if isBalanced(text.root) != text.ans {
			println(index)
		}
	}
}
