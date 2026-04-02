// Package main ...
package main

func maximumAmount(coins [][]int) int {
	n := len(coins[0])
	dp := make([][]int, n+1)
	const minInt = -1 << 31

	for i := range dp {
		dp[i] = make([]int, 3)
		for j := range dp[i] {
			dp[i][j] = minInt / 2
		}
	}

	for i := 0; i < 3; i++ {
		dp[1][i] = 0
	}
	for _, row := range coins {
		for j := 1; j <= n; j++ {
			x := row[j-1]
			dp[j][2] = max(max(dp[j-1][2]+x, dp[j][2]+x),
				max(dp[j-1][1], dp[j][1]))
			dp[j][1] = max(max(dp[j-1][1]+x, dp[j][1]+x),
				max(dp[j-1][0], dp[j][0]))
			dp[j][0] = max(dp[j-1][0], dp[j][0]) + x
		}
	}

	return dp[n][2]
}

func main() {
	tests := []struct {
		coins [][]int
		ans   int
	}{
		{[][]int{{0, 1, -1}, {1, -2, 3}, {2, -3, 4}}, 8},
		{[][]int{{10, 10, 10}, {10, 10, 10}}, 40},
	}

	for _, test := range tests {
		if got := maximumAmount(test.coins); got != test.ans {
			panic(got)
		}
	}
}
