// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxTwoEvents(events [][]int) int {
	type Event struct {
		ts  int
		op  int
		val int
	}

	evs := make([]Event, 0)
	for _, event := range events {
		evs = append(evs, Event{event[0], 0, event[2]})
		evs = append(evs, Event{event[1], 1, event[2]})
	}

	sort.Slice(evs, func(i, j int) bool {
		if evs[i].ts != evs[j].ts {
			return evs[i].ts < evs[j].ts
		}
		return evs[i].op < evs[j].op
	})

	ans, bestFirst := 0, 0
	for _, ev := range evs {
		if ev.op == 0 {
			if ev.val+bestFirst > ans {
				ans = ev.val + bestFirst
			}
		} else {
			if ev.val > bestFirst {
				bestFirst = ev.val
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		events [][]int
		ans    int
	}{
		{[][]int{{1, 3, 2}, {4, 5, 2}, {2, 4, 3}}, 4},
		{[][]int{{1, 3, 2}, {4, 5, 2}, {1, 5, 5}}, 5},
		{[][]int{{1, 5, 3}, {1, 5, 1}, {6, 6, 5}}, 8},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxTwoEvents(test.events), "case %d", index)
	}
}
