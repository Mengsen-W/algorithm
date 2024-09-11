// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximizeWin(prizePositions []int, k int) int {
	n := len(prizePositions)
	dp := make([]int, n+1)
	ans := 0
	for left, right := 0, 0; right < n; right++ {
		for prizePositions[right]-prizePositions[left] > k {
			left++
		}
		ans = max(ans, right-left+1+dp[left])
		dp[right+1] = max(dp[right], right-left+1)
	}

	return ans
}

func main() {
	tests := []struct {
		prizePositions []int
		k              int
		ans            int
	}{
		{[]int{1, 1, 2, 2, 3, 3, 5}, 2, 7},
		{[]int{1, 2, 3, 4}, 0, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximizeWin(test.prizePositions, test.k), index)
	}
}
