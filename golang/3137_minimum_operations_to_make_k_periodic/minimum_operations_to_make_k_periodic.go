// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumOperationsToMakeKPeriodic(word string, k int) int {
	n, res := len(word), math.MaxInt
	count := map[string]int{}
	for i := 0; i < n; i += k {
		count[word[i:i+k]]++
		res = min(res, n/k-count[word[i:i+k]])
	}
	return res
}

func main() {
	tests := []struct {
		word string
		k    int
		ans  int
	}{
		{"leetcodeleet", 4, 1},
		{"leetcoleet", 2, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumOperationsToMakeKPeriodic(test.word, test.k), index)
	}
}
