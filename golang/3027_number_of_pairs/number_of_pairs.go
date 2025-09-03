// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfPairs(points [][]int) int {
	col := make(map[int]int)
	row := make(map[int]int)
	coordinatesMap := make(map[[2]int][2]int)

	for _, point := range points {
		x, y := point[0], point[1]
		col[x] = 0
		row[y] = 0
	}

	colKeys := make([]int, 0, len(col))
	for k := range col {
		colKeys = append(colKeys, k)
	}
	sort.Ints(colKeys)
	for i, key := range colKeys {
		col[key] = i + 1
	}
	rowKeys := make([]int, 0, len(row))
	for k := range row {
		rowKeys = append(rowKeys, k)
	}
	sort.Ints(rowKeys)
	for i, key := range rowKeys {
		row[key] = i + 1
	}

	nc := len(col) + 1
	nr := len(row) + 1

	m := make([][]int, nc)
	prefixSum := make([][]int, nc)
	for i := range m {
		m[i] = make([]int, nr)
		prefixSum[i] = make([]int, nr)
	}

	for _, point := range points {
		x, y := point[0], point[1]
		c, r := col[x], row[y]
		key := [2]int{x, y}
		coordinatesMap[key] = [2]int{c, r}
		m[c][r] = 1
	}

	for i := 1; i < nc; i++ {
		for j := 1; j < nr; j++ {
			prefixSum[i][j] = prefixSum[i-1][j] + prefixSum[i][j-1] -
				prefixSum[i-1][j-1] + m[i][j]
		}
	}

	ans := 0
	sort.Slice(points, func(i, j int) bool {
		if points[i][0] == points[j][0] {
			return points[i][1] > points[j][1]
		}
		return points[i][0] < points[j][0]
	})

	n := len(points)
	for i := 0; i < n-1; i++ {
		for j := i + 1; j < n; j++ {
			if points[i][1] >= points[j][1] {
				key1 := [2]int{points[i][0], points[i][1]}
				key2 := [2]int{points[j][0], points[j][1]}
				coord1 := coordinatesMap[key1]
				coord2 := coordinatesMap[key2]
				c1, r1 := coord1[0], coord1[1]
				c2, r2 := coord2[0], coord2[1]

				cnt := prefixSum[c2][r1] - prefixSum[c1-1][r1] -
					prefixSum[c2][r2-1] + prefixSum[c1-1][r2-1]

				if cnt == 2 {
					ans++
				}
			}
		}
	}

	return ans
}

func main() {
	tests := []struct {
		points [][]int
		ans    int
	}{
		{[][]int{{1, 1}, {2, 2}, {3, 3}}, 0},
		{[][]int{{6, 2}, {4, 4}, {2, 6}}, 2},
		{[][]int{{3, 1}, {1, 3}, {1, 1}}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfPairs(test.points), index)
	}
}
