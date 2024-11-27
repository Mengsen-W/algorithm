// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfAlternatingGroups(colors []int, k int) int {
	n := len(colors)
	res, cnt := 0, 1
	for i := -k + 2; i < n; i++ {
		if colors[(i+n)%n] != colors[(i-1+n)%n] {
			cnt++
		} else {
			cnt = 1
		}
		if cnt >= k {
			res++
		}
	}
	return res
}

func main() {
	tests := []struct {
		colors []int
		k      int
		ans    int
	}{
		{[]int{0, 1, 0, 1, 0}, 3, 3},
		{[]int{0, 1, 0, 0, 1, 0, 1}, 6, 2},
		{[]int{1, 1, 0, 1}, 4, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfAlternatingGroups(test.colors, test.k), index)
	}
}
