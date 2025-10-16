// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findSmallestInteger(nums []int, value int) int {
	mp := make([]int, value)
	for _, x := range nums {
		v := ((x % value) + value) % value
		mp[v]++
	}
	mex := 0
	for mp[mex%value] > 0 {
		mp[mex%value]--
		mex++
	}
	return mex
}

func main() {
	tests := []struct {
		nums  []int
		value int
		ans   int
	}{
		{[]int{1, -10, 7, 13, 6, 8}, 5, 4},
		{[]int{1, -10, 7, 13, 6, 8}, 7, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findSmallestInteger(test.nums, test.value), index)
	}
}
