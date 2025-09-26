// Package main
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func triangleNumber(nums []int) (ans int) {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	n := len(nums)
	sort.Ints(nums)
	for i, v := range nums {
		k := i
		for j := i + 1; j < n; j++ {
			for k+1 < n && nums[k+1] < v+nums[j] {
				k++
			}
			ans += max(k-j, 0)
		}
	}
	return
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 2, 3, 4}, 3},
		{[]int{4, 2, 3, 4}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, triangleNumber(test.nums), index)
	}
}
