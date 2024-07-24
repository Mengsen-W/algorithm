// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func relocateMarbles(nums []int, moveFrom []int, moveTo []int) []int {
	mp := make(map[int]bool)
	var ans []int

	for _, num := range nums {
		mp[num] = true
	}
	for i := range moveFrom {
		delete(mp, moveFrom[i])
		mp[moveTo[i]] = true
	}
	for key := range mp {
		ans = append(ans, key)
	}
	sort.Ints(ans)
	return ans
}

func main() {
	tests := []struct {
		nums     []int
		moveFrom []int
		moveTo   []int
		ans      []int
	}{
		{[]int{1, 6, 7, 8}, []int{1, 7, 2}, []int{2, 9, 5}, []int{5, 6, 8, 9}},
		{[]int{1, 1, 3, 3}, []int{1, 3}, []int{2, 2}, []int{2}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, relocateMarbles(test.nums, test.moveFrom, test.moveTo), index)
	}
}
