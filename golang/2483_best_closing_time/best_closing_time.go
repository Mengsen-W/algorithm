// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func bestClosingTime(customers string) int {
	n := len(customers)
	suf := 0
	pre := 0
	minCost := 0
	res := 0

	for i := 0; i <= n; i++ {
		if minCost > suf+pre {
			minCost = suf + pre
			res = i
		}
		if i < n && customers[i] == 'N' {
			pre++
		} else if i < n {
			suf--
		}
	}
	return res
}

func main() {
	tests := []struct {
		customers string
		ans       int
	}{
		{"YYNY", 2},
		{"NNNNN", 0},
		{"YYYY", 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, bestClosingTime(test.customers), index)
	}
}
