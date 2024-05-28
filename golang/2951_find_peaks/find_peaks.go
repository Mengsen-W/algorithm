// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findPeaks(mountain []int) []int {
	var res []int
	for i := 1; i < len(mountain)-1; i++ {
		if mountain[i-1] < mountain[i] && mountain[i] > mountain[i+1] {
			res = append(res, i)
		}
	}
	return res
}

func main() {
	tests := []struct {
		mountain []int
		ans      []int
	}{
		{[]int{2, 4, 4}, nil},
		{[]int{1, 4, 3, 8, 5}, []int{1, 3}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findPeaks(test.mountain), index)
	}
}
