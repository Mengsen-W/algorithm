// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minOperations(nums []int) int {
	n := len(nums)
	num1 := 0
	g := 0
	for _, x := range nums {
		if x == 1 {
			num1++
		}
		g = gcd(g, x)
	}
	if num1 > 0 {
		return n - num1
	}
	if g > 1 {
		return -1
	}

	minLen := n
	for i := 0; i < n; i++ {
		currentGcd := 0
		for j := i; j < n; j++ {
			currentGcd = gcd(currentGcd, nums[j])
			if currentGcd == 1 {
				if j-i+1 < minLen {
					minLen = j - i + 1
				}
				break
			}
		}
	}
	return minLen + n - 2
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
		{[]int{2, 6, 3, 4}, 4},
		{[]int{2, 10, 6, 14}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minOperations(test.nums), index)
	}
}
