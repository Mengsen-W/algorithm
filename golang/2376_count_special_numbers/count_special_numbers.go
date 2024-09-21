// Package main ...
package main

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func countSpecialNumbers(n int) int {
	nStr := strconv.Itoa(n)
	memo := make(map[int]int)

	var dp func(int, bool, string) int
	dp = func(mask int, prefixSmaller bool, nStr string) int {
		if countOnes(mask) == len(nStr) {
			return 1
		}
		key := mask * 2
		if prefixSmaller {
			key++
		}
		if _, exists := memo[key]; !exists {
			res, lowerBound := 0, 0
			if mask == 0 {
				lowerBound = 1
			}
			upperBound := 9
			if !prefixSmaller {
				upperBound = int(nStr[countOnes(mask)] - '0')
			}
			for i := lowerBound; i <= upperBound; i++ {
				if (mask>>i)&1 == 0 {
					res += dp(mask|(1<<i), prefixSmaller || i < upperBound, nStr)
				}
			}
			memo[key] = res
		}
		return memo[key]
	}

	res, prod := 0, 9
	for i := 0; i < len(nStr)-1; i++ {
		res += prod
		prod *= 9 - i
	}
	res += dp(0, false, nStr)
	return res
}

func countOnes(x int) int {
	count := 0
	for x > 0 {
		count++
		x &= x - 1
	}
	return count
}

func btoi(b bool) int {
	if b {
		return 1
	}
	return 0
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{20, 19},
		{5, 5},
		{135, 110},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countSpecialNumbers(test.n), index)
	}
}
