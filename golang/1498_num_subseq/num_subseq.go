// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func numSubseq(nums []int, target int) int {
	n := len(nums)
	P := int(1e9 + 7)
	f := make([]int, n)
	f[0] = 1
	for i := 1; i < n; i++ {
		f[i] = (f[i-1] * 2) % P
	}
	sort.Ints(nums)
	ans := 0
	for i, num := range nums {
		if num*2 > target {
			break
		}
		maxValue := target - num
		pos := sort.SearchInts(nums, maxValue+1) - 1
		contribute := 0
		if pos >= i {
			contribute = f[pos-i]
		}
		ans = (ans + contribute) % P
	}

	return ans
}

func main() {
	tests := []struct {
		nums   []int
		target int
		ans    int
	}{
		{[]int{3, 5, 6, 7}, 9, 4},
		{[]int{3, 3, 6, 8}, 10, 6},
		{[]int{2, 3, 3, 4, 6, 7}, 12, 61},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numSubseq(test.nums, test.target), index)
	}
}
