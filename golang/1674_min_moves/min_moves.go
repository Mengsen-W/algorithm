// Package main ...
package main

import (
	"fmt"
	"sort"
)

func minMoves(nums []int, limit int) int {
	n := len(nums)
	sumCount := make(map[int]int)
	minArr := make([]int, n/2)
	maxArr := make([]int, n/2)

	for i := 0; i < n/2; i++ {
		a := min(nums[i], nums[n-1-i])
		b := max(nums[i], nums[n-1-i])

		sumCount[a+b]++
		minArr[i] = a
		maxArr[i] = b
	}

	sort.Ints(minArr)
	sort.Ints(maxArr)
	minOps := n

	for c := 2; c <= 2*limit; c++ {
		addLeft := n/2 - lowerBound(minArr, c)
		addRight := lowerBound(maxArr, c-limit)

		currentOps := n/2 + addLeft + addRight - sumCount[c]
		minOps = min(minOps, currentOps)
	}

	return minOps
}

func lowerBound(arr []int, target int) int {
	left, right := 0, len(arr)
	for left < right {
		mid := (left + right) / 2
		if arr[mid] >= target {
			right = mid
		} else {
			left = mid + 1
		}
	}
	return left
}

func main() {
	tests := []struct {
		nums  []int
		limit int
		want  int
	}{
		{[]int{1, 2, 4, 3}, 4, 1},
		{[]int{1, 2, 2, 1}, 2, 2},
		{[]int{1, 2, 1, 2}, 2, 0},
	}

	for _, test := range tests {
		if got := minMoves(test.nums, test.limit); got != test.want {
			panic(fmt.Sprintf("minMoves(%v, %v) = %v, want %v", test.nums, test.limit, got, test.want))
		}
	}
}
