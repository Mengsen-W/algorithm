// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findSpecialInteger(arr []int) int {
	n := len(arr)
	span := n/4 + 1
	for i := 0; i < n; i += span {
		start := binarySearch(arr, arr[i])
		end := binarySearch(arr, arr[i]+1)
		if end-start >= span {
			return arr[i]
		}
	}
	return -1
}

func binarySearch(arr []int, target int) int {
	lo, hi := 0, len(arr)-1
	res := len(arr)
	for lo <= hi {
		mid := (lo + hi) / 2
		if arr[mid] >= target {
			res = mid
			hi = mid - 1
		} else {
			lo = mid + 1
		}
	}
	return res
}

func main() {
	tests := []struct {
		arr []int
		ans int
	}{
		{[]int{1, 2, 2, 6, 6, 6, 6, 7, 10}, 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findSpecialInteger(test.arr), index)
	}
}
