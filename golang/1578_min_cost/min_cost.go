// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minCost(colors string, neededTime []int) int {
	i, n := 0, len(colors)
	ret := 0
	for i < n {
		ch := colors[i]
		maxValue := 0
		sum := 0

		for i < n && colors[i] == ch {
			if neededTime[i] > maxValue {
				maxValue = neededTime[i]
			}
			sum += neededTime[i]
			i++
		}
		ret += sum - maxValue
	}
	return ret
}

func main() {
	tests := []struct {
		colors     string
		neededTime []int
		ans        int
	}{
		{"abaac", []int{1, 2, 3, 4, 5}, 3},
		{"abc", []int{1, 2, 3}, 0},
		{"aabaa", []int{1, 2, 3, 4, 1}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, minCost(test.colors, test.neededTime), test.ans, index)
	}
}
