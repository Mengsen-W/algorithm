/*
 * @Date: 2023-09-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-09
 * @FilePath: /algorithm/golang/207_can_finish/can_finish.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func canFinish(numCourses int, prerequisites [][]int) bool {
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
	return len(result) == numCourses
}

func main() {
	tests := []struct {
		numCourses    int
		prerequisites [][]int
		ans           bool
	}{
		{2, [][]int{{1, 0}}, true},
		{2, [][]int{{1, 0}, {0, 1}}, false},
	}

	for index, item := range tests {
		assert.Equal(&testing.T{}, item.ans, canFinish(item.numCourses, item.prerequisites), index)
	}
}
