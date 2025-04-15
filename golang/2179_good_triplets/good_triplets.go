// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type FenwickTree struct {
	tree []int
}

func fenwickTree(size int) *FenwickTree {
	return &FenwickTree{tree: make([]int, size+1)}
}

func (ft *FenwickTree) update(index, delta int) {
	index++
	for index < len(ft.tree) {
		ft.tree[index] += delta
		index += index & -index
	}
}

func (ft *FenwickTree) query(index int) int {
	index++
	res := 0
	for index > 0 {
		res += ft.tree[index]
		index -= index & -index
	}
	return res
}

func goodTriplets(nums1 []int, nums2 []int) int64 {
	n := len(nums1)
	pos2, reversedIndexMapping := make([]int, n), make([]int, n)
	for i, num := range nums2 {
		pos2[num] = i
	}

	for i, num := range nums1 {
		reversedIndexMapping[pos2[num]] = i
	}
	tree := fenwickTree(n)
	var res int64
	for value := 0; value < n; value++ {
		pos := reversedIndexMapping[value]
		left := tree.query(pos)
		tree.update(pos, 1)
		right := (n - 1 - pos) - (value - left)
		res += int64(left * right)
	}
	return res
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		ans   int64
	}{
		{[]int{2, 0, 1, 3}, []int{0, 1, 2, 3}, 1},
		{[]int{4, 0, 1, 3, 2}, []int{4, 1, 0, 2, 3}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, goodTriplets(test.nums1, test.nums2), index)
	}
}
