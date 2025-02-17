// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func replaceElements(arr []int) []int {
	n := len(arr)
	ans := make([]int, n)
	ans[n-1] = -1
	for i := n - 2; i >= 0; i-- {
		ans[i] = max(ans[i+1], arr[i+1])
	}
	return ans
}

func main() {
	tests := []struct {
		arr []int
		ans []int
	}{
		{[]int{17, 18, 5, 4, 6, 1}, []int{18, 6, 6, 6, 1, -1}},
		{[]int{400}, []int{-1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, replaceElements(test.arr), index)
	}
}
