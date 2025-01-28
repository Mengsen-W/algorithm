// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func getRow(rowIndex int) []int {
	row := make([]int, rowIndex+1)
	row[0] = 1
	for i := 1; i <= rowIndex; i++ {
		row[i] = row[i-1] * (rowIndex - i + 1) / i
	}
	return row
}

func main() {
	tests := []struct {
		rowIndex int
		ans      []int
	}{
		{3, []int{1, 3, 3, 1}},
		{0, []int{1}},
		{1, []int{1, 1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getRow(test.rowIndex), index)
	}
}
