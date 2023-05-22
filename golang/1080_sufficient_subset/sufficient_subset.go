/*
 * @Date: 2023-05-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-22
 * @FilePath: /algorithm/golang/1080_sufficient_subset/sufficient_subset.go
 */

// Package main ...
package main

import "reflect"

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sufficientSubset(root *TreeNode, limit int) *TreeNode {
	haveSufficient := checkSufficientLeaf(root, 0, limit)
	if haveSufficient {
		return root
	} else {
		return nil
	}
}

func checkSufficientLeaf(node *TreeNode, sum int, limit int) bool {
	if node == nil {
		return false
	}
	if node.Left == nil && node.Right == nil {
		return node.Val+sum >= limit
	}
	haveSufficientLeft := checkSufficientLeaf(node.Left, sum+node.Val, limit)
	haveSufficientRight := checkSufficientLeaf(node.Right, sum+node.Val, limit)
	if !haveSufficientLeft {
		node.Left = nil
	}
	if !haveSufficientRight {
		node.Right = nil
	}
	return haveSufficientLeft || haveSufficientRight
}

func levelOrder(root *TreeNode) [][]int {
	ret := [][]int{}
	if root == nil {
		return ret
	}
	q := []*TreeNode{root}
	for i := 0; len(q) > 0; i++ {
		ret = append(ret, []int{})
		p := []*TreeNode{}
		for j := 0; j < len(q); j++ {
			node := q[j]
			ret[i] = append(ret[i], node.Val)
			if node.Left != nil {
				p = append(p, node.Left)
			}
			if node.Right != nil {
				p = append(p, node.Right)
			}
		}
		q = p
	}
	return ret
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		root := &TreeNode{1, &TreeNode{2, &TreeNode{4, &TreeNode{8, nil, nil}, &TreeNode{9, nil, nil}}, &TreeNode{-99, &TreeNode{-99, nil, nil}, &TreeNode{-99, nil, nil}}}, &TreeNode{3, &TreeNode{-99, &TreeNode{12, nil, nil}, &TreeNode{13, nil, nil}}, &TreeNode{7, &TreeNode{-99, nil, nil}, &TreeNode{14, nil, nil}}}}
		limit := 1
		ans := &TreeNode{1, &TreeNode{2, &TreeNode{4, &TreeNode{8, nil, nil}, &TreeNode{9, nil, nil}}, nil}, &TreeNode{3, nil, &TreeNode{7, nil, &TreeNode{14, nil, nil}}}}
		assert(levelOrder(sufficientSubset(root, limit)), levelOrder(ans))
	}

	{
		root := &TreeNode{5, &TreeNode{4, &TreeNode{11, &TreeNode{7, nil, nil}, &TreeNode{1, nil, nil}}, nil}, &TreeNode{8, &TreeNode{17, nil, nil}, &TreeNode{4, &TreeNode{5, nil, nil}, &TreeNode{3, nil, nil}}}}
		limit := 22
		ans := &TreeNode{5, &TreeNode{4, &TreeNode{11, &TreeNode{7, nil, nil}, nil}, nil}, &TreeNode{8, &TreeNode{17, nil, nil}, &TreeNode{4, &TreeNode{5, nil, nil}, nil}}}
		assert(levelOrder(sufficientSubset(root, limit)), levelOrder(ans))
	}

	{
		root := &TreeNode{1, &TreeNode{2, &TreeNode{-5, nil, nil}, nil}, &TreeNode{-3, &TreeNode{4, nil, nil}, nil}}
		limit := -1
		ans := &TreeNode{1, nil, &TreeNode{-3, &TreeNode{4, nil, nil}, nil}}
		assert(levelOrder(sufficientSubset(root, limit)), levelOrder(ans))
	}
}
