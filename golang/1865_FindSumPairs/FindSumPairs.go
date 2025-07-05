// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type FindSumPairs struct {
	nums1 []int
	nums2 []int
	cnt   map[int]int
}

func Constructor(nums1 []int, nums2 []int) FindSumPairs {
	cnt := make(map[int]int)
	for _, num := range nums2 {
		cnt[num]++
	}
	return FindSumPairs{
		nums1: nums1,
		nums2: nums2,
		cnt:   cnt,
	}
}

func (f *FindSumPairs) Add(index int, val int) {
	oldVal := f.nums2[index]
	f.cnt[oldVal]--
	f.nums2[index] += val
	f.cnt[f.nums2[index]]++
}

func (f *FindSumPairs) Count(tot int) int {
	ans := 0
	for _, num := range f.nums1 {
		rest := tot - num
		ans += f.cnt[rest]
	}
	return ans
}

func main() {
	nums1 := []int{1, 1, 2, 2, 2, 3}
	nums2 := []int{1, 4, 5, 2, 5, 4}
	t := &testing.T{}
	findSumPairs := Constructor(nums1, nums2)
	assert.Equal(t, findSumPairs.Count(7), 8)
	findSumPairs.Add(3, 2)
	assert.Equal(t, findSumPairs.Count(8), 2)
	assert.Equal(t, findSumPairs.Count(4), 1)
	findSumPairs.Add(0, 1)
	findSumPairs.Add(1, 1)
	assert.Equal(t, findSumPairs.Count(7), 11)
}
