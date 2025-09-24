// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxCandies(status []int, candies []int, keys [][]int, containedBoxes [][]int, initialBoxes []int) int {
	n := len(status)
	canOpen := make([]bool, n)
	hasBox := make([]bool, n)
	used := make([]bool, n)

	for i := 0; i < n; i++ {
		canOpen[i] = (status[i] == 1)
	}
	q := []int{}
	ans := 0
	for _, box := range initialBoxes {
		hasBox[box] = true
		if canOpen[box] {
			q = append(q, box)
			used[box] = true
			ans += candies[box]
		}
	}

	for len(q) > 0 {
		bigBox := q[0]
		q = q[1:]
		for _, key := range keys[bigBox] {
			canOpen[key] = true
			if !used[key] && hasBox[key] {
				q = append(q, key)
				used[key] = true
				ans += candies[key]
			}
		}
		for _, box := range containedBoxes[bigBox] {
			hasBox[box] = true
			if !used[box] && canOpen[box] {
				q = append(q, box)
				used[box] = true
				ans += candies[box]
			}
		}
	}

	return ans
}

func main() {
	tests := []struct {
		status         []int
		candies        []int
		keys           [][]int
		containedBoxes [][]int
		initialBoxes   []int
		ans            int
	}{
		{[]int{1, 0, 1, 0}, []int{7, 5, 4, 100}, [][]int{{}, {}, {1}, {}}, [][]int{{1, 2}, {3}, {}, {}}, []int{0}, 16},
		{[]int{1, 0, 0, 0, 0, 0}, []int{1, 1, 1, 1, 1, 1}, [][]int{{1, 2, 3, 4, 5}, {}, {}, {}, {}, {}}, [][]int{{1, 2, 3, 4, 5}, {}, {}, {}, {}, {}}, []int{0}, 6},
		{[]int{1, 1, 1}, []int{100, 1, 100}, [][]int{{}, {0, 2}, {}}, [][]int{{}, {}, {}}, []int{1}, 1},
		{[]int{1}, []int{100}, [][]int{{}}, [][]int{{}}, []int{}, 0},
		{[]int{1, 1, 1}, []int{2, 3, 2}, [][]int{{}, {}, {}}, [][]int{{}, {}, {}}, []int{2, 1, 0}, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxCandies(test.status, test.candies, test.keys, test.containedBoxes, test.initialBoxes), index)
	}
}
