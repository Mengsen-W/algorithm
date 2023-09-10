/*
 * @Date: 2023-09-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-10
 * @FilePath: /algorithm/golang/210_find_order/find_order.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findOrder(numCourses int, prerequisites [][]int) []int {
	var (
		edges  = make([][]int, numCourses)
		indeg  = make([]int, numCourses)
		result []int
	)

	for _, info := range prerequisites {
		edges[info[1]] = append(edges[info[1]], info[0])
		indeg[info[0]]++
	}

	q := []int{}
	for i := 0; i < numCourses; i++ {
		if indeg[i] == 0 {
			q = append(q, i)
		}
	}

	for len(q) > 0 {
		u := q[0]
		q = q[1:]
		result = append(result, u)
		for _, v := range edges[u] {
			indeg[v]--
			if indeg[v] == 0 {
				q = append(q, v)
			}
		}
	}
	if len(result) != numCourses {
		return []int{}
	}
	return result
}

func main() {
	tests := []struct {
		numCourses    int
		prerequisites [][]int
		result        []int
	}{
		{2, [][]int{{1, 0}}, []int{0, 1}},
		{1, [][]int{}, []int{0}},
	}

	for index, item := range tests {
		assert.Equal(&testing.T{}, item.result, findOrder(item.numCourses, item.prerequisites), index)
	}
}
