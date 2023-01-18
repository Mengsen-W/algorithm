/*
 * @Date: 2022-05-20 22:19:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-20 22:27:37
 * @FilePath: /algorithm/436_find_right_interval/find_right_interval.go
 */

package main

import (
	"reflect"
	"sort"
)

func findRightInterval(intervals [][]int) []int {
	n := len(intervals)
	type pair struct{ x, i int }
	starts := make([]pair, n)
	ends := make([]pair, n)
	for i, p := range intervals {
		starts[i] = pair{p[0], i}
		ends[i] = pair{p[1], i}
	}
	sort.Slice(starts, func(i, j int) bool { return starts[i].x < starts[j].x })
	sort.Slice(ends, func(i, j int) bool { return ends[i].x < ends[j].x })

	ans := make([]int, n)
	j := 0
	for _, p := range ends {
		for j < n && starts[j].x < p.x {
			j++
		}
		if j < n {
			ans[p.i] = starts[j].i
		} else {
			ans[p.i] = -1
		}
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("NOt Passed")
		}
	}

	assert(findRightInterval([][]int{{1, 2}}), []int{-1})
	assert(findRightInterval([][]int{{3, 4}, {2, 3}, {1, 2}}), []int{-1, 0, 1})
	assert(findRightInterval([][]int{{1, 4}, {2, 3}, {3, 4}}), []int{-1, 2, -1})
}
