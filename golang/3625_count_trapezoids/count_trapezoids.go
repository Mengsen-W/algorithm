// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countTrapezoids(points [][]int) int {
	n := len(points)
	inf := 1e9 + 7
	slopeToIntercept := make(map[float64][]float64)
	midToSlope := make(map[float64][]float64)
	ans := 0

	for i := 0; i < n; i++ {
		x1, y1 := points[i][0], points[i][1]
		for j := i + 1; j < n; j++ {
			x2, y2 := points[j][0], points[j][1]
			dx := x1 - x2
			dy := y1 - y2

			var k, b float64
			if x2 == x1 {
				k = inf
				b = float64(x1)
			} else {
				k = float64(y2-y1) / float64(x2-x1)
				b = float64(y1*dx-x1*dy) / float64(dx)
			}

			mid := float64((x1+x2)*10000 + (y1 + y2))
			slopeToIntercept[k] = append(slopeToIntercept[k], b)
			midToSlope[mid] = append(midToSlope[mid], k)
		}
	}

	for _, sti := range slopeToIntercept {
		if len(sti) == 1 {
			continue
		}

		cnt := make(map[float64]int)
		for _, bVal := range sti {
			cnt[bVal]++
		}

		totalSum := 0
		for _, count := range cnt {
			ans += totalSum * count
			totalSum += count
		}
	}

	for _, mts := range midToSlope {
		if len(mts) == 1 {
			continue
		}

		cnt := make(map[float64]int)
		for _, kVal := range mts {
			cnt[kVal]++
		}

		totalSum := 0
		for _, count := range cnt {
			ans -= totalSum * count
			totalSum += count
		}
	}

	return ans
}

func main() {
	tests := []struct {
		points   [][]int
		expected int
	}{
		{[][]int{{-3, 2}, {3, 0}, {2, 3}, {3, 2}, {2, -3}}, 2},
		{[][]int{{0, 0}, {1, 0}, {0, 1}, {2, 1}}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expected, countTrapezoids(test.points), index)
	}
}
