// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func superEggDrop(k int, n int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	dp := make([]int, n+1)
	for i := 0; i <= n; i++ {
		dp[i] = i
	}

	for j := 2; j <= k; j++ {
		dp2 := make([]int, n+1)
		x := 1
		dp2[0] = 0
		for m := 1; m <= n; m++ {
			for x < m && max(dp[x-1], dp2[m-x]) >= max(dp[x], dp2[m-x-1]) {
				x++
			}
			dp2[m] = 1 + max(dp[x-1], dp2[m-x])
		}
		copy(dp, dp2)
	}
	return dp[n]
}

func main() {
	tests := []struct {
		k   int
		n   int
		ans int
	}{
		{1, 2, 2},
		{2, 6, 3},
		{3, 14, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, superEggDrop(test.n, test.n), index)
	}
}
