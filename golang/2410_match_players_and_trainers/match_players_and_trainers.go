// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func matchPlayersAndTrainers(players []int, trainers []int) (ans int) {
	sort.Ints(players)
	sort.Ints(trainers)
	m, n := len(players), len(trainers)
	for i, j := 0, 0; i < m && j < n; i++ {
		for j < n && players[i] > trainers[j] {
			j++
		}
		if j < n {
			ans++
			j++
		}
	}
	return
}

func main() {
	tests := []struct {
		players  []int
		trainers []int
		ans      int
	}{
		{[]int{4, 7, 9}, []int{8, 2, 5, 8}, 2},
		{[]int{1, 1, 1}, []int{10}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, matchPlayersAndTrainers(test.players, test.trainers), index)
	}
}
