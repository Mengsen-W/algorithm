/*
 * @Date: 2024-04-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-24
 * @FilePath: /algorithm/golang/2385_amount_of_time/amount_of_time.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func amountOfTime(root *TreeNode, start int) int {
	graph := map[int][]int{}
	var dfs func(*TreeNode)
	dfs = func(node *TreeNode) {
		for _, child := range []*TreeNode{node.Left, node.Right} {
			if child != nil {
				graph[node.Val] = append(graph[node.Val], child.Val)
				graph[child.Val] = append(graph[child.Val], node.Val)
				dfs(child)
			}
		}
	}
	dfs(root)
	q := [][]int{{start, 0}}
	visited := map[int]bool{}
	visited[start] = true
	time := 0
	for len(q) > 0 {
		arr := q[0]
		q = q[1:]
		nodeVal := arr[0]
		time = arr[1]
		for _, childVal := range graph[nodeVal] {
			if !visited[childVal] {
				q = append(q, []int{childVal, time + 1})
				visited[childVal] = true
			}
		}
	}
	return time
}

func main() {
	tests := []struct {
		root  *TreeNode
		start int
		ans   int
	}{
		{
			&TreeNode{
				1, &TreeNode{5, nil, &TreeNode{4, &TreeNode{9, nil, nil}, &TreeNode{2, nil, nil}}},
				&TreeNode{3, &TreeNode{10, nil, nil}, &TreeNode{6, nil, nil}},
			},
			3,
			4,
		},
		{
			&TreeNode{1, nil, nil},
			1,
			0,
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, amountOfTime(test.root, test.start), index)
	}
}
