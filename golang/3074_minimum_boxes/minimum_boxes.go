// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumBoxes(apple []int, capacity []int) int {
	sum := 0
	for _, a := range apple {
		sum += a
	}

	sort.Sort(sort.Reverse(sort.IntSlice(capacity)))
	need := 0
	for sum > 0 {
		sum -= capacity[need]
		need++
	}

	return need
}

func main() {
	tests := []struct {
		apple    []int
		capacity []int
		ans      int
	}{
		{[]int{1, 3, 2}, []int{4, 3, 1, 5, 2}, 2},
		{[]int{5, 5, 5}, []int{2, 4, 2, 7}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumBoxes(test.apple, test.capacity), index)
	}
}
