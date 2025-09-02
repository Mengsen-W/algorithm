// Package main ,,,
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfPairs(points [][]int) int {
	ans := 0
	n := len(points)

	for i := 0; i < n; i++ {
		pointA := points[i]
		for j := 0; j < n; j++ {
			pointB := points[j]
			if i == j || (pointA[0] > pointB[0] || pointA[1] < pointB[1]) {
				continue
			}
			if n == 2 {
				ans++
				continue
			}

			illegal := false
			for k := 0; k < n; k++ {
				if k == i || k == j {
					continue
				}

				pointTmp := points[k]
				isXContained := pointTmp[0] >= pointA[0] && pointTmp[0] <= pointB[0]
				isYContained := pointTmp[1] <= pointA[1] && pointTmp[1] >= pointB[1]
				if isXContained && isYContained {
					illegal = true
					break
				}
			}
			if !illegal {
				ans++
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		points   [][]int
		expected int
	}{
		{[][]int{{1, 1}, {2, 2}, {3, 3}}, 0},
		{[][]int{{6, 2}, {4, 4}, {2, 6}}, 2},
		{[][]int{{3, 1}, {1, 3}, {1, 1}}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expected, numberOfPairs(test.points), index)
	}
}
