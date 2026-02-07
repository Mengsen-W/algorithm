// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumDeletions(s string) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	leftb := 0
	righta := 0
	for _, c := range s {
		if c == 'a' {
			righta++
		}
	}
	res := righta
	for _, c := range s {
		if c == 'a' {
			righta--
		} else {
			leftb++
		}
		res = min(res, leftb+righta)
	}
	return res
}

func main() {
	tests := []struct {
		s   string
		res int
	}{
		{"aababbab", 2},
		{"bbaaaaabb", 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.res, minimumDeletions(test.s), index)
	}
}
