/*
 * @Date: 2024-01-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-28
 * @FilePath: /algorithm/golang/365_can_measure_water/can_measure_water.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func canMeasureWater(x int, y int, z int) bool {
	if x+y < z {
		return false
	}
	if x == 0 || y == 0 {
		return z == 0 || x+y == z
	}
	return z%gcd(x, y) == 0
}

func gcd(x int, y int) int {
	if y == 0 {
		return x
	}
	return gcd(y, x%y)
}

func main() {
	tests := []struct {
		x   int
		y   int
		z   int
		ans bool
	}{
		{3, 5, 4, true},
		{2, 6, 5, false},
		{1, 2, 3, true},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, canMeasureWater(test.x, test.y, test.z), index)
	}
}
