// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func sumZero(n int) []int {
	ans := make([]int, 0, n)
	for i := 1; i <= n/2; i++ {
		ans = append(ans, i, -i)
	}
	if n%2 == 1 {
		ans = append(ans, 0)
	}
	return ans
}

func check(ans []int) bool {
	seen := make(map[int]struct{})
	sum := 0
	for _, item := range ans {
		if _, exists := seen[item]; !exists {
			seen[item] = struct{}{}
		} else {
			return false
		}
		sum += item
	}
	return sum == 0
}

func main() {
	tests := []int{5, 3, 1}

	for _, n := range tests {
		ans := sumZero(n)
		assert.True(&testing.T{}, check(ans))
	}
}
