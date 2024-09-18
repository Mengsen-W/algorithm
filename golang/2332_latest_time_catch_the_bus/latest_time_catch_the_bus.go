// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func latestTimeCatchTheBus(buses []int, passengers []int, capacity int) int {
	sort.Ints(buses)
	sort.Ints(passengers)
	pos := 0
	space := 0

	for _, arrive := range buses {
		space = capacity
		for space > 0 && pos < len(passengers) && passengers[pos] <= arrive {
			space--
			pos++
		}
	}

	pos--
	lastCatchTime := buses[len(buses)-1]
	if space <= 0 {
		lastCatchTime = passengers[pos]
	}
	for pos >= 0 && passengers[pos] == lastCatchTime {
		pos--
		lastCatchTime--
	}

	return lastCatchTime
}

func main() {
	tests := []struct {
		buses      []int
		passengers []int
		capacity   int
		ans        int
	}{
		{[]int{10, 20}, []int{2, 17, 18, 19}, 2, 16},
		{[]int{20, 30, 10}, []int{19, 13, 26, 4, 25, 11, 21}, 2, 20},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, latestTimeCatchTheBus(test.buses, test.passengers, test.capacity), index)
	}
}
