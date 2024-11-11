// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countKConstraintSubstrings(s string, k int) int {
	n := len(s)
	res := 0
	for i := 0; i < n; i++ {
		count := [2]int{}
		for j := i; j < n; j++ {
			count[int(s[j]-'0')]++
			if count[0] > k && count[1] > k {
				break
			}
			res++
		}
	}
	return res
}

func main() {
	tests := []struct {
		s   string
		k   int
		ans int
	}{
		{"10101", 1, 12},
		{"1010101", 2, 25},
		{"11111", 1, 15},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countKConstraintSubstrings(test.s, test.k), index)
	}
}
