// Package main ...
package main

import "sort"

func minimumEffort(tasks [][]int) int {
	sort.Slice(tasks, func(i, j int) bool {
		return tasks[i][1]-tasks[i][0] > tasks[j][1]-tasks[j][0]
	})
	ans := 0
	remain := 0
	for _, task := range tasks {
		if remain > task[1] {
			// 不需要增加能量
		} else {
			ans += task[1] - remain
		}
		if task[1]-task[0] > remain-task[0] {
			remain = task[1] - task[0]
		} else {
			remain = remain - task[0]
		}
	}
	return ans
}

func main() {
	tests := []struct {
		tasks [][]int
		ans   int
	}{
		{[][]int{{1, 2}, {2, 4}, {4, 8}}, 8},
		{[][]int{{1, 3}, {2, 4}, {10, 11}, {10, 12}, {8, 9}}, 32},
		{[][]int{{1, 7}, {2, 8}, {3, 9}, {4, 10}, {5, 11}, {6, 12}}, 27},
	}

	for _, test := range tests {
		if minimumEffort(test.tasks) != test.ans {
			panic("error")
		}
	}
}
