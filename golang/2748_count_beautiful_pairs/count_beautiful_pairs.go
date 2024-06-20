// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countBeautifulPairs(nums []int) int {
	res := 0
	n := len(nums)
	for i := 0; i < n; i++ {
		for nums[i] >= 10 {
			nums[i] /= 10
		}
		for j := i + 1; j < n; j++ {
			if gcd(nums[i], nums[j]%10) == 1 {
				res++
			}
		}
	}
	return res
}

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 5, 1, 4}, 5},
		{[]int{11, 21, 12}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countBeautifulPairs(test.nums), index)
	}
}
