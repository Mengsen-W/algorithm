// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func diagonalPrime(nums [][]int) int {
	n, res := len(nums), 0
	for i := 0; i < n; i++ {
		if isPrime(nums[i][i]) {
			res = max(res, nums[i][i])
		}
		if isPrime(nums[i][n-i-1]) {
			res = max(res, nums[i][n-i-1])
		}
	}
	return res
}

func isPrime(num int) bool {
	if num == 1 {
		return false
	}
	factor := 2
	for ; factor*factor <= num; factor++ {
		if num%factor == 0 {
			return false
		}
	}
	return true
}

func main() {
	tests := []struct {
		nums [][]int
		ans  int
	}{
		{[][]int{{1, 2, 3}, {5, 6, 7}, {9, 10, 11}}, 11},
		{[][]int{{1, 2, 3}, {5, 17, 7}, {9, 11, 10}}, 17},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, diagonalPrime(test.nums), index)
	}
}
