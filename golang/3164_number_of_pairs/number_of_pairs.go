// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfPairs(nums1 []int, nums2 []int, k int) int64 {
	count := make(map[int]int)
	count2 := make(map[int]int)
	max1 := 0
	for _, num := range nums1 {
		count[num]++
		if num > max1 {
			max1 = num
		}
	}
	for _, num := range nums2 {
		count2[num]++
	}
	var res int64
	for a, cnt := range count2 {
		for b := a * k; b <= max1; b += a * k {
			if _, ok := count[b]; ok {
				res += int64(count[b] * cnt)
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		k     int
		ans   int64
	}{
		{[]int{1, 3, 4}, []int{1, 3, 4}, 1, 5},
		{[]int{1, 2, 4, 12}, []int{2, 4}, 3, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfPairs(test.nums1, test.nums2, test.k), index)
	}
}
