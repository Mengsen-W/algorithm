// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countTrapezoids(points [][]int) int {
	pointNum := make(map[int]int)
	mod := 1000000007
	ans, sum := 0, 0

	for _, point := range points {
		y := point[1]
		pointNum[y]++
	}

	for _, pNum := range pointNum {
		edge := pNum * (pNum - 1) / 2
		ans = (ans + edge*sum) % mod
		sum = (sum + edge) % mod
	}

	return ans
}

func main() {
	tests := []struct {
		points [][]int
		ans    int
	}{
		{[][]int{{1, 0}, {2, 0}, {3, 0}, {2, 2}, {3, 2}}, 3},
		{[][]int{{0, 0}, {1, 0}, {0, 1}, {2, 1}}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countTrapezoids(test.points), "index: %d", index)
	}
}
