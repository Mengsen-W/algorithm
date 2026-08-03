// Package main ...
package main

import (
	"fmt"
	"math"
)

func stoneGameIII(stoneValue []int) string {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	n := len(stoneValue)
	dp := make([]int, n+1)
	for i := 0; i <= n; i++ {
		dp[i] = math.MinInt
	}
	dp[n] = 0
	for i := n - 1; i >= 0; i-- {
		currentSum := 0
		for j := i + 1; j <= i+3 && j <= n; j++ {
			currentSum += stoneValue[j-1]
			dp[i] = max(dp[i], currentSum-dp[j])
		}
	}

	if dp[0] > 0 {
		return "Alice"
	}
	if dp[0] < 0 {
		return "Bob"
	}
	return "Tie"
}

func main() {
	tests := []struct {
		stoneValue []int
		ans        string
	}{
		{[]int{1, 2, 3, 7}, "Bob"},
		{[]int{1, 2, 3, -9}, "Alice"},
		{[]int{1, 2, 3, 6}, "Tie"},
	}

	for index, test := range tests {
		if stoneGameIII(test.stoneValue) != test.ans {
			fmt.Println("test failed", index, stoneGameIII(test.stoneValue), test.ans)
		}
	}
}
