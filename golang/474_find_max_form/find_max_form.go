// Package main ...
package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func findMaxForm(strs []string, m, n int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	dp := make([][]int, m+1)
	for i := range dp {
		dp[i] = make([]int, n+1)
	}
	for _, s := range strs {
		zeros := strings.Count(s, "0")
		ones := len(s) - zeros
		for j := m; j >= zeros; j-- {
			for k := n; k >= ones; k-- {
				dp[j][k] = max(dp[j][k], dp[j-zeros][k-ones]+1)
			}
		}
	}
	return dp[m][n]
}

func main() {
	tests := []struct {
		strs []string
		m, n int
		ans  int
	}{
		{[]string{"10", "0001", "111001", "1", "0"}, 5, 3, 4},
		{[]string{"01", "0", "1"}, 1, 1, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findMaxForm(test.strs, test.m, test.n), index)
	}
}
