// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxFreeTime(eventTime int, startTime []int, endTime []int) int {
	n := len(startTime)
	q := make([]bool, n)
	t1, t2 := 0, 0
	for i := 0; i < n; i++ {
		if endTime[i]-startTime[i] <= t1 {
			q[i] = true
		}
		if i == 0 {
			t1 = max(t1, startTime[i])
		} else {
			t1 = max(t1, startTime[i]-endTime[i-1])
		}

		if endTime[n-i-1]-startTime[n-i-1] <= t2 {
			q[n-i-1] = true
		}
		if i == 0 {
			t2 = max(t2, eventTime-endTime[n-1])
		} else {
			t2 = max(t2, startTime[n-i]-endTime[n-i-1])
		}
	}

	res := 0
	for i := 0; i < n; i++ {
		left := 0
		if i != 0 {
			left = endTime[i-1]
		}
		right := eventTime
		if i != n-1 {
			right = startTime[i+1]
		}
		if q[i] {
			res = max(res, right-left)
		} else {
			res = max(res, right-left-(endTime[i]-startTime[i]))
		}
	}
	return res
}

func main() {
	tests := []struct {
		eventTime int
		startTime []int
		endTime   []int
		ans       int
	}{
		{5, []int{1, 3}, []int{2, 5}, 2},
		{10, []int{0, 7, 9}, []int{1, 8, 10}, 7},
		{10, []int{0, 3, 7, 9}, []int{1, 4, 8, 10}, 6},
		{5, []int{0, 1, 2, 3, 4}, []int{1, 2, 3, 4, 5}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxFreeTime(test.eventTime, test.startTime, test.endTime), index)
	}
}
