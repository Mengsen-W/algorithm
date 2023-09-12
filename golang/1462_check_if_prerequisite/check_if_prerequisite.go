/*
 * @Date: 2023-09-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-12
 * @FilePath: /algorithm/golang/1462_check_if_prerequisite/check_if_prerequisite.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func checkIfPrerequisite(numCourses int, prerequisites [][]int, queries [][]int) []bool {
	g := make([][]int, numCourses)
	indgree := make([]int, numCourses)
	isPre := make([][]bool, numCourses)
	for i := range isPre {
		isPre[i] = make([]bool, numCourses)
		g[i] = []int{}
	}
	for _, p := range prerequisites {
		indgree[p[1]]++
		g[p[0]] = append(g[p[0]], p[1])
	}

	q := []int{}
	for i := 0; i < numCourses; i++ {
		if indgree[i] == 0 {
			q = append(q, i)
		}
	}

	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		for _, ne := range g[cur] {
			isPre[cur][ne] = true
			for i := 0; i < numCourses; i++ {
				isPre[i][ne] = isPre[i][ne] || isPre[i][cur]
			}
			indgree[ne]--
			if indgree[ne] == 0 {
				q = append(q, ne)
			}
		}
	}
	res := []bool{}
	for _, query := range queries {
		res = append(res, isPre[query[0]][query[1]])
	}
	return res
}

func main() {
	tests := []struct {
		numCourses    int
		prerequisites [][]int
		queries       [][]int
		ans           []bool
	}{
		{2, [][]int{{1, 0}}, [][]int{{0, 1}, {1, 0}}, []bool{false, true}},
		{2, [][]int{}, [][]int{{1, 0}, {0, 1}}, []bool{false, false}},
		{3, [][]int{{1, 2}, {1, 0}, {2, 0}}, [][]int{{1, 0}, {1, 2}}, []bool{true, true}},
	}

	for index, item := range tests {
		assert.Equal(&testing.T{}, item.ans, checkIfPrerequisite(item.numCourses, item.prerequisites, item.queries), index)
	}
}
