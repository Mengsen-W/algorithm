// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numEquivDominoPairs(dominoes [][]int) (ans int) {
	cnt := [100]int{}
	for _, d := range dominoes {
		if d[0] > d[1] {
			d[0], d[1] = d[1], d[0]
		}
		v := d[0]*10 + d[1]
		ans += cnt[v]
		cnt[v]++
	}
	return
}

func main() {
	tests := []struct {
		dominoes [][]int
		ans      int
	}{
		{[][]int{{1, 2}, {2, 1}, {3, 4}, {5, 6}}, 1},
		{[][]int{{1, 2}, {1, 2}, {1, 1}, {1, 2}, {2, 2}}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numEquivDominoPairs(test.dominoes), index)
	}
}
