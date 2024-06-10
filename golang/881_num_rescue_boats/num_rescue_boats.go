// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func numRescueBoats(people []int, limit int) (ans int) {
	sort.Ints(people)
	light, heavy := 0, len(people)-1
	for light <= heavy {
		if people[light]+people[heavy] > limit {
			heavy--
		} else {
			light++
			heavy--
		}
		ans++
	}
	return
}

func main() {
	tests := []struct {
		people []int
		limit  int
		ans    int
	}{
		{[]int{1, 2}, 3, 1},
		{[]int{3, 2, 2, 1}, 3, 3},
		{[]int{3, 5, 3, 4}, 5, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numRescueBoats(test.people, test.limit), index)
	}
}
