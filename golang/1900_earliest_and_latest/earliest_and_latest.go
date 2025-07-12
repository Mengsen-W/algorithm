// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func earliestAndLatest(n int, firstPlayer int, secondPlayer int) []int {
	const maxN = 30
	var F [maxN][maxN][maxN]int
	var G [maxN][maxN][maxN]int
	if firstPlayer > secondPlayer {
		firstPlayer, secondPlayer = secondPlayer, firstPlayer
	}

	var dp func(n, f, s int) (int, int)
	dp = func(n, f, s int) (int, int) {
		if F[n][f][s] != 0 {
			return F[n][f][s], G[n][f][s]
		}
		if f+s == n+1 {
			return 1, 1
		}
		// F(n,f,s) = F(n,n+1-s,n+1-f)
		if f+s > n+1 {
			x, y := dp(n, n+1-s, n+1-f)
			F[n][f][s] = x
			G[n][f][s] = y
			return x, y
		}

		earlist := math.MaxInt32
		latest := math.MinInt32
		n_half := (n + 1) / 2
		if s <= n_half {
			// 在左侧或者中间
			for i := 0; i < f; i++ {
				for j := 0; j < s-f; j++ {
					x, y := dp(n_half, i+1, i+j+2)
					earlist = min(earlist, x)
					latest = max(latest, y)
				}
			}
		} else {
			// s 在右侧
			sPrime := n + 1 - s
			mid := (n - 2*sPrime + 1) / 2
			for i := 0; i < f; i++ {
				for j := 0; j < sPrime-f; j++ {
					x, y := dp(n_half, i+1, i+j+mid+2)
					earlist = min(earlist, x)
					latest = max(latest, y)
				}
			}
		}

		F[n][f][s] = earlist + 1
		G[n][f][s] = latest + 1
		return F[n][f][s], G[n][f][s]
	}

	earlist, latest := dp(n, firstPlayer, secondPlayer)
	return []int{earlist, latest}
}

func main() {
	tests := []struct {
		n            int
		firstPlayer  int
		secondPlayer int
		ans          []int
	}{
		{11, 2, 4, []int{3, 4}},
		{5, 1, 5, []int{1, 1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, earliestAndLatest(test.n, test.firstPlayer, test.secondPlayer), index)
	}
}
