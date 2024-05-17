/*
 * @Date: 2024-05-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-17
 * @FilePath: /algorithm/golang/826_max_profit_assignment/max_profit_assignment.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxProfitAssignment(difficulty []int, profit []int, worker []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	jobs := make([][2]int, len(difficulty))
	for i := range difficulty {
		jobs[i] = [2]int{difficulty[i], profit[i]}
	}
	sort.Slice(jobs, func(i, j int) bool { return jobs[i][0] < jobs[j][0] })
	sort.Ints(worker)

	res, i, best := 0, 0, 0
	for _, w := range worker {
		for i < len(jobs) && w >= jobs[i][0] {
			best = max(best, jobs[i][1])
			i++
		}
		res += best
	}
	return res
}

func main() {
	tests := []struct {
		difficulty []int
		profit     []int
		worker     []int
		ans        int
	}{
		{[]int{2, 4, 6, 8, 10}, []int{10, 20, 30, 40, 50}, []int{4, 5, 6, 7}, 100},
		{[]int{85, 47, 57}, []int{24, 66, 99}, []int{40, 25, 25}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxProfitAssignment(test.difficulty, test.profit, test.worker), index)
	}
}
