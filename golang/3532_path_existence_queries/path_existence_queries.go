// Package main ...
package main

import (
	"fmt"
	"reflect"
)

func pathExistenceQueries(n int, nums []int, maxDiff int, queries [][]int) []bool {
	tags := make([]int, n)
	for i := 1; i < n; i++ {
		if nums[i]-nums[i-1] > maxDiff {
			tags[i] = tags[i-1] + 1
		} else {
			tags[i] = tags[i-1]
		}
	}

	res := make([]bool, len(queries))
	for i, query := range queries {
		x, y := query[0], query[1]
		res[i] = tags[x] == tags[y]
	}
	return res
}

func main() {
	tests := []struct {
		n       int
		nums    []int
		maxDiff int
		queries [][]int
		ans     []bool
	}{
		{2, []int{1, 3}, 1, [][]int{{0, 0}, {0, 1}}, []bool{true, false}},
		{4, []int{2, 5, 6, 8}, 2, [][]int{{0, 1}, {0, 2}, {1, 3}, {2, 3}}, []bool{false, false, true, true}},
	}

	for index, test := range tests {
		if !reflect.DeepEqual(test.ans, pathExistenceQueries(test.n, test.nums, test.maxDiff, test.queries)) {
			fmt.Println(index)
		}
	}
}
