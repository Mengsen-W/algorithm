// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const MOD = 1000000007

func numberOfStableArrays(zero int, one int, limit int) int {
	dp := make([][][]int, zero+1)
	for i := range dp {
		dp[i] = make([][]int, one+1)
		for j := range dp[i] {
			dp[i][j] = make([]int, 2)
		}
	}

	for i := 0; i <= zero; i++ {
		for j := 0; j <= one; j++ {
			for lastBit := 0; lastBit <= 1; lastBit++ {
				if i == 0 {
					if lastBit == 0 || j > limit {
						dp[i][j][lastBit] = 0
					} else {
						dp[i][j][lastBit] = 1
					}
				} else if j == 0 {
					if lastBit == 1 || i > limit {
						dp[i][j][lastBit] = 0
					} else {
						dp[i][j][lastBit] = 1
					}
				} else if lastBit == 0 {
					dp[i][j][lastBit] = dp[i-1][j][0] + dp[i-1][j][1]
					if i > limit {
						dp[i][j][lastBit] -= dp[i-limit-1][j][1]
					}
				} else {
					dp[i][j][lastBit] = dp[i][j-1][0] + dp[i][j-1][1]
					if j > limit {
						dp[i][j][lastBit] -= dp[i][j-limit-1][0]
					}
				}
				dp[i][j][lastBit] %= MOD
				if dp[i][j][lastBit] < 0 {
					dp[i][j][lastBit] += MOD
				}
			}
		}
	}

	return (dp[zero][one][0] + dp[zero][one][1]) % MOD
}

func main() {
	tests := []struct {
		zero  int
		one   int
		limit int
		ans   int
	}{
		{1, 1, 2, 2},
		{1, 2, 1, 1},
		{3, 3, 2, 14},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfStableArrays(test.zero, test.one, test.limit), index)
	}
}
