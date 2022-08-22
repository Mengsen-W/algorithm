/*
 * @Date: 2022-08-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-22
 * @FilePath: /algorithm/655_print_tree/print_tree.go
 */

package main

import (
	"reflect"
	"strconv"
)

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func calDepth(root *TreeNode) int {
	h := -1
	q := []*TreeNode{root}
	for len(q) > 0 {
		h++
		tmp := q
		q = nil
		for _, node := range tmp {
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
	}
	return h
}

func printTree(root *TreeNode) [][]string {
	height := calDepth(root)
	m := height + 1
	n := 1<<m - 1
	ans := make([][]string, m)
	for i := range ans {
		ans[i] = make([]string, n)
	}
	type entry struct {
		node *TreeNode
		r, c int
	}
	q := []entry{{root, 0, (n - 1) / 2}}
	for len(q) > 0 {
		e := q[0]
		q = q[1:]
		node, r, c := e.node, e.r, e.c
		ans[r][c] = strconv.Itoa(node.Val)
		if node.Left != nil {
			q = append(q, entry{node.Left, r + 1, c - 1<<(height-r-1)})
		}
		if node.Right != nil {
			q = append(q, entry{node.Right, r + 1, c + 1<<(height-r-1)})
		}
	}
	return ans
}

func main() {
	assert := func(a, b [][]string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		root := &TreeNode{1, &TreeNode{2, nil, nil}, nil}
		ans := [][]string{{"", "1", ""}, {"2", "", ""}}
		assert(printTree(root), ans)
	}

	{
		root := &TreeNode{1, &TreeNode{2, nil, &TreeNode{4, nil, nil}}, &TreeNode{3, nil, nil}}
		ans := [][]string{{"", "", "", "1", "", "", ""}, {"", "2", "", "", "", "3", ""}, {"", "", "4", "", "", "", ""}}
		assert(printTree(root), ans)
	}
}
