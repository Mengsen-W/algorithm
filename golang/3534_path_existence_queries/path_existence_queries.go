// Package main ...
package main

import (
	"fmt"
	"reflect"
	"sort"
)

func pathExistenceQueries(n int, nums []int, maxDiff int, queries [][]int) []int {
	idx := make([]int, n)
	pos := make([]int, n)

	for i := 0; i < n; i++ {
		idx[i] = i
	}

	sort.Slice(idx, func(i, j int) bool {
		return nums[idx[i]] < nums[idx[j]]
	})

	for i := 0; i < n; i++ {
		pos[idx[i]] = i
	}

	m := 0
	for t := n; t > 0; t >>= 1 {
		m++
	}

	f := make([][]int, n)
	for i := range f {
		f[i] = make([]int, m)
	}

	left := 0

	for i := 0; i < n; i++ {
		for left < i &&
			nums[idx[i]]-nums[idx[left]] > maxDiff {
			left++
		}

		f[i][0] = left
	}

	for j := 1; j < m; j++ {
		for i := 0; i < n; i++ {
			f[i][j] = f[f[i][j-1]][j-1]
		}
	}

	res := make([]int, 0, len(queries))

	for _, q := range queries {

		x := pos[q[0]]
		y := pos[q[1]]

		if x > y {
			x, y = y, x
		}

		if x == y {
			res = append(res, 0)
			continue
		}

		step := 0

		for i := m - 1; i >= 0; i-- {
			if f[y][i] > x {
				y = f[y][i]
				step += 1 << i
			}
		}

		if f[y][0] <= x {
			res = append(res, step+1)
		} else {
			res = append(res, -1)
		}
	}

	return res
}

func main() {
	tests := []struct {
		n       int
		nums    []int
		maxDiff int
		queries [][]int
		ans     []int
	}{
		{4, []int{1, 8, 3, 4, 2}, 3, [][]int{{0, 3}, {2, 4}}, []int{1, 1}},
		{5, []int{5, 3, 1, 9, 10}, 2, [][]int{{0, 1}, {0, 2}, {2, 3}, {4, 3}}, []int{1, 2, -1, 1}},
		{3, []int{3, 6, 1}, 1, [][]int{{0, 0}, {0, 1}, {1, 2}}, []int{0, -1, -1}},
	}

	for _, test := range tests {
		res := pathExistenceQueries(test.n, test.nums, test.maxDiff, test.queries)
		if !reflect.DeepEqual(res, test.ans) {
			fmt.Println("test failed", test)
		}
	}
}
