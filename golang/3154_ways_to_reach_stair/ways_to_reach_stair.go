// Package main ,,,
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func waysToReachStair(k int) int {
	n, npow, ans := 0, 1, 0
	for {
		if npow-n-1 <= k && k <= npow {
			ans += comb(n+1, npow-k)
		} else if npow-n-1 > k {
			break
		}
		n++
		npow *= 2
	}
	return ans
}

func comb(n, k int) int {
	ans := 1
	for i := n; i >= n-k+1; i-- {
		ans *= i
		ans /= n - i + 1
	}
	return ans
}

func main() {
	tests := []struct {
		k   int
		ans int
	}{
		{0, 2},
		{1, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, waysToReachStair(test.k), index)
	}
}
