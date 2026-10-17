// Package main ...
package main

import (
	"sort"
	"strconv"
)

func minSwaps(nums []int) int {
	n := len(nums)
	digitSum := make([]int, n)
	for i, v := range nums {
		sum := 0
		for v > 0 {
			sum += v % 10
			v /= 10
		}
		digitSum[i] = sum
	}

	indices := make([]int, n)
	for i := 0; i < n; i++ {
		indices[i] = i
	}
	sort.Slice(indices, func(a, b int) bool {
		if digitSum[indices[a]] != digitSum[indices[b]] {
			return digitSum[indices[a]] < digitSum[indices[b]]
		}
		return nums[indices[a]] < nums[indices[b]]
	})

	targetPos := make([]int, n)
	for pos, idx := range indices {
		targetPos[idx] = pos
	}

	parent := make([]int, n)
	for i := 0; i < n; i++ {
		parent[i] = i
	}
	components := n

	var find func(int) int
	find = func(x int) int {
		if parent[x] != x {
			parent[x] = find(parent[x])
		}
		return parent[x]
	}

	for i := 0; i < n; i++ {
		ri := find(i)
		rj := find(targetPos[i])
		if ri != rj {
			parent[ri] = rj
			components--
		}
	}

	return n - components
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{37, 100}, 1},
		{[]int{22, 14, 33, 7}, 0},
		{[]int{18, 43, 34, 16}, 2},
	}

	for _, test := range tests {
		if got := minSwaps(test.nums); got != test.ans {
			panic("got " + strconv.Itoa(got) + " want " + strconv.Itoa(test.ans))
		}
	}
}
