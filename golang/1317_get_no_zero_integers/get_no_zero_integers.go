// Package main ...
package main

import (
	"strconv"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func getNoZeroIntegers(n int) []int {
	for A := 1; A < n; A++ {
		B := n - A
		if !strings.Contains(strconv.Itoa(A), "0") && !strings.Contains(strconv.Itoa(B), "0") {
			return []int{A, B}
		}
	}
	return []int{}
}

func main() {
	tests := []struct {
		n   int
		ans []int
	}{
		{2, []int{1, 1}},
		{11, []int{2, 9}},
		{10000, []int{1, 9999}},
		{69, []int{1, 68}},
		{1010, []int{11, 999}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getNoZeroIntegers(test.n), index)
	}
}
