// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findKthNumber(n, k int) int {
	getSteps := func(cur, n int) (steps int) {
		min := func(a, b int) int {
			if a > b {
				return b
			}
			return a
		}
		first, last := cur, cur
		for first <= n {
			steps += min(last, n) - first + 1
			first *= 10
			last = last*10 + 9
		}
		return
	}
	cur := 1
	k--
	for k > 0 {
		steps := getSteps(cur, n)
		if steps <= k {
			k -= steps
			cur++
		} else {
			cur *= 10
			k--
		}
	}
	return cur
}

func main() {
	tests := []struct {
		n   int
		k   int
		ans int
	}{
		{13, 2, 10},
		{1, 1, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findKthNumber(test.n, test.k), index)
	}
}
