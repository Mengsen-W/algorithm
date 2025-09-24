// Package main ...
package main

import (
	"container/list"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxTaskAssign(tasks []int, workers []int, pills int, strength int) int {
	n, m := len(tasks), len(workers)
	sort.Ints(tasks)
	sort.Ints(workers)

	check := func(mid int) bool {
		p := pills
		ws := list.New() // 使用双端队列
		ptr := m - 1
		// 从大到小枚举每一个任务
		for i := mid - 1; i >= 0; i-- {
			for ptr >= m-mid && workers[ptr]+strength >= tasks[i] {
				ws.PushFront(workers[ptr]) // 添加到队头
				ptr--
			}
			if ws.Len() == 0 {
				return false
			}
			// 如果双端队列中最大的元素大于等于 tasks[i]
			if ws.Back().Value.(int) >= tasks[i] {
				ws.Remove(ws.Back()) // 移除队尾
			} else {
				if p == 0 {
					return false
				}
				p--
				ws.Remove(ws.Front()) // 移除队头
			}
		}
		return true
	}

	left, right, ans := 1, min(m, n), 0
	for left <= right {
		mid := (left + right) / 2
		if check(mid) {
			ans = mid
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
	return ans
}

func main() {
	tests := []struct {
		tasks    []int
		workers  []int
		pills    int
		strength int
		ans      int
	}{
		{[]int{3, 2, 1}, []int{0, 3, 3}, 1, 1, 3},
		{[]int{5, 4}, []int{0, 0, 0}, 1, 5, 1},
		{[]int{10, 15, 30}, []int{0, 10, 10, 10, 10}, 3, 10, 2},
		{[]int{5, 9, 8, 5, 9}, []int{1, 6, 4, 2, 6}, 1, 5, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxTaskAssign(test.tasks, test.workers, test.pills, test.strength), index)
	}
}
