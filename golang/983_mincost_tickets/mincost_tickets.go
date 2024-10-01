// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func mincostTickets(days []int, costs []int) int {
	max := func(i, j int) int {
		if i > j {
			return i
		}
		return j
	}

	min := func(i, j, k int) int {
		if j < i {
			i = j
		}
		if k < i {
			return k
		}
		return i
	}

	maxDay := days[len(days)-1] + 30
	ans := make([]int, maxDay)
	for _, day := range days {
		ans[day]++
	}
	for day := 1; day < maxDay; day++ {
		if ans[day] > 0 {
			ans[day] = min(
				ans[day-1]+costs[0],
				ans[max(day-7, 0)]+costs[1],
				ans[max(day-30, 0)]+costs[2],
			)
		} else {
			ans[day] = ans[day-1]
		}
	}
	return ans[maxDay-1]
}

func main() {
	tests := []struct {
		days  []int
		costs []int
		ans   int
	}{
		{[]int{1, 4, 6, 7, 8, 20}, []int{2, 7, 15}, 11},
		{[]int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31}, []int{2, 7, 15}, 17},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, mincostTickets(test.days, test.costs), index)
	}
}
