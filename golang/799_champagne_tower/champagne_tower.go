// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func champagneTower(poured, queryRow, queryGlass int) float64 {
	row := []float64{float64(poured)}
	for i := 1; i <= queryRow; i++ {
		nextRow := make([]float64, i+1)
		for j, volume := range row {
			if volume > 1 {
				nextRow[j] += (volume - 1) / 2
				nextRow[j+1] += (volume - 1) / 2
			}
		}
		row = nextRow
	}
	return math.Min(1, row[queryGlass])
}

func main() {
	tests := []struct {
		poured     int
		queryRow   int
		queryGlass int
		ans        float64
	}{
		{1, 1, 1, 0.0},
		{2, 1, 1, 0.5},
		{100000009, 33, 17, 1.0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, champagneTower(test.poured, test.queryRow, test.queryGlass), index)
	}
}
