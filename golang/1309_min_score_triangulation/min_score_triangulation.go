// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minScoreTriangulation(values []int) int {
	memo := make(map[int]int)
	n := len(values)
	var dp func(int, int) int
	dp = func(i int, j int) int {
		if i+2 > j {
			return 0
		}
		if i+2 == j {
			return values[i] * values[i+1] * values[j]
		}
		key := i*n + j
		if _, ok := memo[key]; !ok {
			minScore := math.MaxInt32
			for k := i + 1; k < j; k++ {
				minScore = min(minScore, values[i]*values[k]*values[j]+dp(i, k)+dp(k, j))
			}
			memo[key] = minScore
		}
		return memo[key]
	}
	return dp(0, n-1)
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	tests := []struct {
		values []int
		want   int
	}{
		{[]int{1, 2, 3}, 6},
		{[]int{3, 7, 4, 5}, 144},
		{[]int{1, 3, 1, 4, 1, 5}, 13},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.want, minScoreTriangulation(test.values), "test case %d failed", index)
	}
}
