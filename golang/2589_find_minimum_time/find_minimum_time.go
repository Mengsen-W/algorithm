/*
 * @Date: 2024-05-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-15
 * @FilePath: /algorithm/golang/2589_find_minimum_time/find_minimum_time.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func findMinimumTime(tasks [][]int) int {
	sort.Slice(tasks, func(i, j int) bool {
		return tasks[i][1] < tasks[j][1]
	})
	var st [][]int
	st = append(st, []int{-1, -1, 0})
	for _, task := range tasks {
		start, end, duration := task[0], task[1], task[2]
		k := sort.Search(len(st), func(i int) bool {
			return st[i][0] >= start
		})
		duration -= st[len(st)-1][2] - st[k-1][2]
		if start <= st[k-1][1] {
			duration -= st[k-1][1] - start + 1
		}
		if duration <= 0 {
			continue
		}
		for end-st[len(st)-1][1] <= duration {
			duration += st[len(st)-1][1] - st[len(st)-1][0] + 1
			st = st[:len(st)-1]
		}
		st = append(st, []int{end - duration + 1, end, st[len(st)-1][2] + duration})
	}
	return st[len(st)-1][2]
}

func main() {
	tests := []struct {
		tasks [][]int
		ans   int
	}{
		{[][]int{{2, 3, 1}, {4, 5, 1}, {1, 5, 2}}, 2},
		{[][]int{{1, 3, 2}, {2, 5, 3}, {5, 6, 2}}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findMinimumTime(test.tasks), index)
	}
}
