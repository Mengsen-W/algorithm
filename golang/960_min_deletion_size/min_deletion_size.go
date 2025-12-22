// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minDeletionSize(strs []string) int {
	n := len(strs[0])
	dp := make([]int, n)
	for i := range dp {
		dp[i] = 1
	}

	for i := n - 2; i >= 0; i-- {
		for j := i + 1; j < n; j++ {
			valid := true
			for _, row := range strs {
				if row[i] > row[j] {
					valid = false
					break
				}
			}
			if valid {
				if dp[i] < 1+dp[j] {
					dp[i] = 1 + dp[j]
				}
			}
		}
	}

	maxVal := 0
	for _, val := range dp {
		maxVal = max(maxVal, val)
	}

	return n - maxVal
}

func main() {
	tests := []struct {
		strs []string
		ans  int
	}{
		{[]string{"babca", "bbazb"}, 3},
		{[]string{"edcba"}, 4},
		{[]string{"ghi", "def", "abc"}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minDeletionSize(test.strs), index)
	}
}
