/*
 * @Date: 2022-05-01 09:27:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-01 10:03:55
 * @FilePath: /algorithm/1305_get_all_elements_in_two_bst/get_all_elements.go
 */

package main

import "reflect"

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func inorder(root *TreeNode) (res []int) {
	var dfs func(*TreeNode)
	dfs = func(node *TreeNode) {
		if node == nil {
			return
		}
		dfs(node.Left)
		res = append(res, node.Val)
		dfs(node.Right)
	}
	dfs(root)
	return
}

func getAllElements(root1, root2 *TreeNode) []int {
	nums1 := inorder(root1)
	nums2 := inorder(root2)

	p1, n1 := 0, len(nums1)
	p2, n2 := 0, len(nums2)
	merged := make([]int, 0, n1+n2)
	for {
		if p1 == n1 {
			return append(merged, nums2[p2:]...)
		}
		if p2 == n2 {
			return append(merged, nums1[p1:]...)
		}
		if nums1[p1] < nums2[p2] {
			merged = append(merged, nums1[p1])
			p1++
		} else {
			merged = append(merged, nums2[p2])
			p2++
		}
	}
}

func main() {
	assert := func(a, b []int) {
		if reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		root1 := &TreeNode{1, &TreeNode{1, nil, nil}, &TreeNode{4, nil, nil}}
		root2 := &TreeNode{1, &TreeNode{0, nil, nil}, &TreeNode{3, nil, nil}}
		assert(getAllElements(root1, root2), []int{0, 1, 1, 2, 3, 4})
	}

	{
		root1 := &TreeNode{1, nil, &TreeNode{8, nil, nil}}
		root2 := &TreeNode{1, &TreeNode{8, nil, nil}, nil}
		assert(getAllElements(root1, root2), []int{1, 1, 8, 8})
	}
}
