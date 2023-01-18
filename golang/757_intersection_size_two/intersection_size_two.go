/*
 * @Date: 2022-07-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-22
 * @FilePath: /algorithm/757_intersection_size_two/intersection_size_two.go
 */

package main

import "sort"

func intersectionSizeTwo(intervals [][]int) (ans int) {
	sort.Slice(intervals, func(i, j int) bool {
		a, b := intervals[i], intervals[j]
		return a[0] < b[0] || a[0] == b[0] && a[1] > b[1]
	})
	n, m := len(intervals), 2
	vals := make([][]int, n)
	for i := n - 1; i >= 0; i-- {
		for j, k := intervals[i][0], len(vals[i]); k < m; k++ {
			ans++
			for p := i - 1; p >= 0 && intervals[p][1] >= j; p-- {
				vals[p] = append(vals[p], j)
			}
			j++
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		intervals := [][]int{{1, 3}, {1, 4}, {2, 5}, {3, 5}}
		assert(intersectionSizeTwo(intervals) == 3)
	}

	{
		intervals := [][]int{{1, 2}, {2, 3}, {2, 4}, {4, 5}}
		assert(intersectionSizeTwo(intervals) == 5)
	}
}
