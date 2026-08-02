// Package main ...
package main

func stoneGame(piles []int) bool {
	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}
	length := len(piles)
	dp := make([]int, length)
	for i := length - 1; i >= 0; i-- {
		dp[i] = piles[i]
		for j := i + 1; j < length; j++ {
			dp[j] = max(piles[i]-dp[j], piles[j]-dp[j-1])
		}
	}
	return dp[length-1] > 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	tests := []struct {
		piles []int
		ans   bool
	}{
		{[]int{5, 3, 4, 5}, true},
		{[]int{3, 7, 2, 3}, true},
	}

	for _, test := range tests {
		assert(stoneGame(test.piles) == test.ans)
	}
}
