// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxFreeTime(eventTime int, k int, startTime []int, endTime []int) int {
	n := len(startTime)
	res := 0
	t := 0
	for i := 0; i < n; i++ {
		t += endTime[i] - startTime[i]
		var left int
		if i <= k-1 {
			left = 0
		} else {
			left = endTime[i-k]
		}
		var right int
		if i == n-1 {
			right = eventTime
		} else {
			right = startTime[i+1]
		}
		if right-left-t > res {
			res = right - left - t
		}
		if i >= k-1 {
			t -= endTime[i-k+1] - startTime[i-k+1]
		}
	}
	return res
}

func main() {
	tests := []struct {
		eventTime int
		k         int
		startTime []int
		endTime   []int
		ans       int
	}{
		{5, 1, []int{1, 3}, []int{2, 5}, 2},
		{10, 1, []int{0, 2, 9}, []int{1, 4, 10}, 6},
		{5, 2, []int{0, 1, 2, 3, 4}, []int{1, 2, 3, 4, 5}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxFreeTime(test.eventTime, test.k, test.startTime, test.endTime), index)
	}
}
