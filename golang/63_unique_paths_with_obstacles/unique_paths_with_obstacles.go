// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func uniquePathsWithObstacles(obstacleGrid [][]int) int {
	n, m := len(obstacleGrid), len(obstacleGrid[0])
	f := make([]int, m)
	if obstacleGrid[0][0] == 0 {
		f[0] = 1
	}
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			if obstacleGrid[i][j] == 1 {
				f[j] = 0
				continue
			}
			if j-1 >= 0 && obstacleGrid[i][j-1] == 0 {
				f[j] += f[j-1]
			}
		}
	}
	return f[len(f)-1]
}

func main() {
	tests := []struct {
		obstacleGrid [][]int
		ans          int
	}{
		{[][]int{{0, 0, 0}, {0, 1, 0}, {0, 0, 0}}, 2},
		{[][]int{{0, 1}, {0, 0}}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, uniquePathsWithObstacles(test.obstacleGrid), index)
	}
}
