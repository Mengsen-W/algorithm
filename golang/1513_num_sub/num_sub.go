// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numSub(s string) int {
	const MODULO = 1000000007
	total := 0
	consecutive := 0
	for _, c := range s {
		if c == '0' {
			total += consecutive * (consecutive + 1) / 2
			total %= MODULO
			consecutive = 0
		} else {
			consecutive++
		}
	}
	total += consecutive * (consecutive + 1) / 2
	total %= MODULO
	return total
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"0110111", 9},
		{"101", 2},
		{"111111", 21},
		{"000", 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numSub(test.s), index)
	}
}
