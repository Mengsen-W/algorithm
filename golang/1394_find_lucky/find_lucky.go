// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findLucky(arr []int) int {
	m := make(map[int]int)
	for _, x := range arr {
		m[x]++
	}
	ans := -1
	for key, value := range m {
		if key == value {
			ans = max(ans, key)
		}
	}
	return ans
}

func main() {
	tests := []struct {
		arr []int
		ans int
	}{
		{[]int{2, 2, 3, 4}, 2},
		{[]int{1, 2, 2, 3, 3, 3}, 3},
		{[]int{2, 2, 2, 3, 3}, -1},
		{[]int{5}, -1},
		{[]int{7, 7, 7, 7, 7, 7, 7}, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findLucky(test.arr), index)
	}
}
