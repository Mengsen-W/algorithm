// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findWinningPlayer(skills []int, k int) int {
	n := len(skills)
	cnt := 0
	i, lastI := 0, 0

	for i < n {
		j := i + 1
		for j < n && skills[j] < skills[i] && cnt < k {
			j++
			cnt++
		}
		if cnt == k {
			return i
		}
		cnt = 1
		lastI = i
		i = j
	}
	return lastI
}

func main() {
	tests := []struct {
		skills []int
		k      int
		ans    int
	}{
		{[]int{4, 2, 6, 3, 9}, 2, 2},
		{[]int{2, 5, 4}, 3, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findWinningPlayer(test.skills, test.k), index)
	}
}
