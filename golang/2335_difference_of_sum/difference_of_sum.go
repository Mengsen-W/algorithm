// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func differenceOfSum(nums []int) (ans int) {
	for _, x := range nums {
		ans += x // 累加元素和
		for x > 0 {
			ans -= x % 10 // 减去数位和
			x /= 10
		}
	}
	return
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 15, 6, 3}, 9},
		{[]int{1, 2, 3, 4}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, differenceOfSum(test.nums), index)
	}
}
