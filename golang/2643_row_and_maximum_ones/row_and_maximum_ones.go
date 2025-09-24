package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func rowAndMaximumOnes(mat [][]int) []int {
	maxOnes := 0
	rowIndex := 0
	for i, row := range mat {
		tot := 0
		for _, val := range row {
			tot += val
		}
		if tot > maxOnes {
			maxOnes = tot
			rowIndex = i
		}
	}
	return []int{rowIndex, maxOnes}
}

func main() {
	tests := []struct {
		mat [][]int
		ans []int
	}{
		{[][]int{{0, 1}, {1, 0}}, []int{0, 1}},
		{[][]int{{0, 0, 0}, {0, 1, 1}}, []int{1, 2}},
		{[][]int{{0, 0}, {1, 1}, {0, 0}}, []int{1, 2}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, rowAndMaximumOnes(test.mat), index)
	}
}
