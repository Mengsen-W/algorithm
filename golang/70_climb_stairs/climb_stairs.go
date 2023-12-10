/*
 * @Date: 2023-12-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-10
 * @FilePath: /algorithm/golang/70_climb_stairs/climb_stairs.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func climbStairs(n int) int {
	sqrt5 := math.Sqrt(5)
	return int(math.Round((math.Pow((1+sqrt5)/2, float64(n+1)) - math.Pow((1-sqrt5)/2, float64(n+1))) / sqrt5))
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{2, 2},
		{3, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, climbStairs(test.n), index)
	}
}
