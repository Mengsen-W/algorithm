// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func threeConsecutiveOdds(arr []int) bool {
	n := len(arr)
	for i := 0; i <= n-3; i++ {
		if (arr[i]&1) != 0 && (arr[i+1]&1) != 0 && (arr[i+2]&1) != 0 {
			return true
		}
	}
	return false
}

func main() {
	tests := []struct {
		arr []int
		ans bool
	}{
		{[]int{2, 6, 4, 1}, false},
		{[]int{1, 2, 34, 3, 4, 5, 7, 23, 12}, true},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, threeConsecutiveOdds(test.arr), index)
	}
}
