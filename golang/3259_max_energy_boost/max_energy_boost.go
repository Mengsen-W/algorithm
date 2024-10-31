// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxEnergyBoost(energyDrinkA []int, energyDrinkB []int) int64 {
	n := len(energyDrinkA)
	d := make([][2]int64, n+1)
	for i := 1; i <= n; i++ {
		d[i][0] = d[i-1][0] + int64(energyDrinkA[i-1])
		d[i][1] = d[i-1][1] + int64(energyDrinkB[i-1])
		if i >= 2 {
			d[i][0] = max(d[i][0], d[i-2][1]+int64(energyDrinkA[i-1]))
			d[i][1] = max(d[i][1], d[i-2][0]+int64(energyDrinkB[i-1]))
		}
	}
	return max(d[n][0], d[n][1])
}

func main() {
	tests := []struct {
		energyDrinkA []int
		energyDrinkB []int
		ans          int64
	}{
		{[]int{1, 3, 1}, []int{3, 1, 1}, 5},
		{[]int{4, 1, 1}, []int{1, 1, 3}, 7},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxEnergyBoost(test.energyDrinkA, test.energyDrinkB), index)
	}
}
