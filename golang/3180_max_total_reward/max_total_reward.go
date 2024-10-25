// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxTotalReward(rewardValues []int) int {
	sort.Ints(rewardValues)
	m := rewardValues[len(rewardValues)-1]
	dp := make([]int, 2*m)
	dp[0] = 1
	for _, x := range rewardValues {
		for k := 2*x - 1; k >= x; k-- {
			if dp[k-x] == 1 {
				dp[k] = 1
			}
		}
	}
	res := 0
	for i := 0; i < len(dp); i++ {
		if dp[i] == 1 {
			res = i
		}
	}
	return res
}

func main() {
	tests := []struct {
		rewardValues []int
		ans          int
	}{
		{[]int{1, 1, 3, 3}, 4},
		{[]int{1, 6, 4, 3, 2}, 11},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxTotalReward(test.rewardValues), index)
	}
}
