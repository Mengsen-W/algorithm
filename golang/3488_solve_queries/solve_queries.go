// Package main ...
package main

import (
	"fmt"
	"reflect"
)

func solveQueries(nums []int, queries []int) []int {
	n := len(nums)
	left := make([]int, n)
	right := make([]int, n)
	pos := make(map[int]int)

	for i := -n; i < n; i++ {
		if i >= 0 {
			left[i] = pos[nums[i]]
		}
		pos[nums[(i+n)%n]] = i
	}

	pos = make(map[int]int)
	for i := 2*n - 1; i >= 0; i-- {
		if i < n {
			right[i] = pos[nums[i]]
		}
		pos[nums[i%n]] = i
	}

	for i := 0; i < len(queries); i++ {
		x := queries[i]
		if x-left[x] == n {
			queries[i] = -1
		} else {
			queries[i] = min(x-left[x], right[x]-x)
		}
	}

	return queries
}

func main() {
	tests := []struct {
		nums    []int
		queries []int
		ans     []int
	}{
		{[]int{1, 3, 1, 4, 1, 3, 2}, []int{0, 3, 5}, []int{2, -1, 3}},
		{[]int{1, 2, 3, 4}, []int{0, 1, 2, 3}, []int{-1, -1, -1, -1}},
	}

	for index, test := range tests {
		if reflect.DeepEqual(solveQueries(test.nums, test.queries), test.ans) {
			fmt.Errorf("%d", index)
		}
	}
}
