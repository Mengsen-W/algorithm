// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func successfulPairs(spells []int, potions []int, success int64) []int {
	res := make([]int, len(spells))
	idx := make([]int, len(spells))
	for i := range idx {
		idx[i] = i
	}
	sort.Slice(potions, func(i, j int) bool {
		return potions[i] > potions[j]
	})
	sort.Slice(idx, func(i, j int) bool {
		return spells[idx[i]] < spells[idx[j]]
	})
	j := 0
	for _, p := range idx {
		v := spells[p]
		for j < len(potions) && int64(potions[j])*int64(v) >= success {
			j++
		}
		res[p] = j
	}
	return res
}

func main() {
	tests := []struct {
		spells  []int
		potions []int
		success int64
		ans     []int
	}{
		{[]int{5, 1, 3}, []int{1, 2, 3, 4, 5}, 7, []int{4, 0, 3}},
		{[]int{3, 1, 2}, []int{8, 5, 8}, 16, []int{2, 0, 2}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, successfulPairs(test.spells, test.potions, test.success), index)
	}
}
