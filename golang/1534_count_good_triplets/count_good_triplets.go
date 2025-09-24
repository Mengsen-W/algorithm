// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countGoodTriplets(arr []int, a int, b int, c int) int {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	ans := 0
	n := len(arr)
	sum := make([]int, 1001)
	for j := 0; j < n; j++ {
		for k := j + 1; k < n; k++ {
			if abs(arr[j]-arr[k]) <= b {
				lj, rj := arr[j]-a, arr[j]+a
				lk, rk := arr[k]-c, arr[k]+c
				l := max(0, max(lj, lk))
				r := min(1000, min(rj, rk))
				if l <= r {
					if l == 0 {
						ans += sum[r]
					} else {
						ans += sum[r] - sum[l-1]
					}
				}
			}
		}
		for k := arr[j]; k <= 1000; k++ {
			sum[k]++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		arr []int
		a   int
		b   int
		c   int
		ans int
	}{
		{[]int{3, 0, 1, 1, 9, 7}, 7, 2, 3, 4},
		{[]int{1, 1, 2, 2, 3}, 0, 0, 1, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countGoodTriplets(test.arr, test.a, test.b, test.c), index)
	}
}
