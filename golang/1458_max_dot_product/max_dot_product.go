// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxDotProduct(nums1 []int, nums2 []int) int {
	m := len(nums1)
	n := len(nums2)
	f := make([][]int, m)
	for i := range f {
		f[i] = make([]int, n)
	}

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			xij := nums1[i] * nums2[j]
			f[i][j] = xij
			if i > 0 {
				f[i][j] = max(f[i][j], f[i-1][j])
			}
			if j > 0 {
				f[i][j] = max(f[i][j], f[i][j-1])
			}
			if i > 0 && j > 0 {
				f[i][j] = max(f[i][j], f[i-1][j-1]+xij)
			}
		}
	}

	return f[m-1][n-1]
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		ans   int
	}{
		{[]int{2, 1, -2, 5}, []int{3, 0, -6}, 18},
		{[]int{3, -2}, []int{2, -6, 7}, 21},
		{[]int{-1, -1}, []int{1, 1}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxDotProduct(test.nums1, test.nums2), index)
	}
}
