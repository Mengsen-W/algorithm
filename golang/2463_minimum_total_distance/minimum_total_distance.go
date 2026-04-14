// Package main ...
package main

import (
	"fmt"
	"math"
	"sort"
)

func abs(x int64) int64 {
	if x < 0 {
		return -x
	}
	return x
}

func minimumTotalDistance(robot []int, factory [][]int) int64 {
	sort.Ints(robot)
	sort.Slice(factory, func(i, j int) bool { return factory[i][0] < factory[j][0] })
	n, m := len(robot), len(factory)
	dp := make([][]int64, n+1)
	for i := range dp {
		dp[i] = make([]int64, m+1)
		if i == 0 {
			continue
		}
		for j := range dp[i] {
			dp[i][j] = math.MaxInt64 / 2
		}
	}
	for j := 1; j <= m; j++ {
		for i := 1; i <= n; i++ {
			dp[i][j] = dp[i][j-1]
			cost := int64(0)
			for k := 1; k <= min(i, factory[j-1][1]); k++ {
				cost += abs(int64(robot[i-k] - factory[j-1][0]))
				dp[i][j] = min(dp[i][j], dp[i-k][j-1]+cost)
			}
		}
	}
	return dp[n][m]
}

func main() {
	tests := []struct {
		robot   []int
		factory [][]int
		ans     int64
	}{
		{[]int{0, 4, 6}, [][]int{{2, 2}, {6, 2}}, 4},
		{[]int{1, -1}, [][]int{{-2, 1}, {2, 1}}, 2},
	}

	for index, test := range tests {
		if minimumTotalDistance(test.robot, test.factory) != test.ans {
			fmt.Errorf("%d", index)
		}
	}
}
