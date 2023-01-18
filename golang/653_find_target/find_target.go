/*
 * @Date: 2022-03-21 00:26:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-21 04:31:35
 * @FilePath: /algorithm/653_find_target/find_target.go
 */

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findTarget(root *TreeNode, k int) bool {
	left, right := root, root
	leftStk := []*TreeNode{left}
	for left.Left != nil {
		leftStk = append(leftStk, left.Left)
		left = left.Left
	}
	rightStk := []*TreeNode{right}
	for right.Right != nil {
		rightStk = append(rightStk, right.Right)
		right = right.Right
	}
	for left != right {
		sum := left.Val + right.Val
		if sum == k {
			return true
		}
		if sum < k {
			left = leftStk[len(leftStk)-1]
			leftStk = leftStk[:len(leftStk)-1]
			for node := left.Right; node != nil; node = node.Left {
				leftStk = append(leftStk, node)
			}
		} else {
			right = rightStk[len(rightStk)-1]
			rightStk = rightStk[:len(rightStk)-1]
			for node := right.Left; node != nil; node = node.Right {
				rightStk = append(rightStk, node)
			}
		}
	}
	return false
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	root := &TreeNode{Val: 5, Left: &TreeNode{Val: 3, Left: &TreeNode{Val: 2}, Right: &TreeNode{Val: 4}}, Right: &TreeNode{Val: 6, Right: &TreeNode{Val: 7}}}
	assert(findTarget(root, 9) == true)
	assert(findTarget(root, 28) == false)
}
