/*
 * @Date: 2023-10-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-31
 * @FilePath: /algorithm/golang/2003_smallest_missing_value_subtree/smallest_missing_value_subtree.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func smallestMissingValueSubtree(parents []int, nums []int) []int {
	n := len(parents)
	children := make([][]int, n)
	for i := 1; i < n; i++ {
		children[parents[i]] = append(children[parents[i]], i)
	}

	geneSet, visited := make(map[int]bool), make([]bool, n)
	var dfs func(int)
	dfs = func(node int) {
		if visited[node] {
			return
		}
		visited[node] = true
		geneSet[nums[node]] = true
		for _, child := range children[node] {
			dfs(child)
		}
	}

	res, node, iNode := make([]int, n), -1, 1
	for i := 0; i < n; i++ {
		res[i] = 1
		if nums[i] == 1 {
			node = i
		}
	}
	for node != -1 {
		dfs(node)
		for geneSet[iNode] {
			iNode++
		}
		res[node], node = iNode, parents[node]
	}
	return res
}

func main() {
	tests := []struct {
		parents []int
		nums    []int
		ans     []int
	}{
		{[]int{-1, 0, 0, 2}, []int{1, 2, 3, 4}, []int{5, 1, 1, 1}},
		{[]int{-1, 0, 1, 0, 3, 3}, []int{5, 4, 6, 2, 1, 3}, []int{7, 1, 1, 4, 2, 1}},
		{[]int{-1, 2, 3, 0, 2, 4, 1}, []int{2, 3, 4, 5, 6, 7, 8}, []int{1, 1, 1, 1, 1, 1, 1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, smallestMissingValueSubtree(test.parents, test.nums), index)
	}
}
