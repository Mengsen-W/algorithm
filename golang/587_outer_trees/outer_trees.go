/*
 * @Date: 2022-04-23 09:49:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-23 10:21:02
 * @FilePath: /algorithm/587_outer_trees/outer_trees.go
 */

package main

import (
	"reflect"
	"sort"
)

func cross(p, q, r []int) int {
	return (q[0]-p[0])*(r[1]-q[1]) - (q[1]-p[1])*(r[0]-q[0])
}

func outerTrees(trees [][]int) [][]int {
	n := len(trees)
	if n < 4 {
		return trees
	}

	// 按照 x 从小到大排序，如果 x 相同，则按照 y 从小到大排序
	sort.Slice(trees, func(i, j int) bool { a, b := trees[i], trees[j]; return a[0] < b[0] || a[0] == b[0] && a[1] < b[1] })

	hull := []int{0} // hull[0] 需要入栈两次，不标记
	used := make([]bool, n)
	// 求凸包的下半部分
	for i := 1; i < n; i++ {
		for len(hull) > 1 && cross(trees[hull[len(hull)-2]], trees[hull[len(hull)-1]], trees[i]) < 0 {
			used[hull[len(hull)-1]] = false
			hull = hull[:len(hull)-1]
		}
		used[i] = true
		hull = append(hull, i)
	}
	// 求凸包的上半部分
	m := len(hull)
	for i := n - 2; i >= 0; i-- {
		if !used[i] {
			for len(hull) > m && cross(trees[hull[len(hull)-2]], trees[hull[len(hull)-1]], trees[i]) < 0 {
				used[hull[len(hull)-1]] = false
				hull = hull[:len(hull)-1]
			}
			used[i] = true
			hull = append(hull, i)
		}
	}
	// hull[0] 同时参与凸包的上半部分检测，因此需去掉重复的 hull[0]
	hull = hull[:len(hull)-1]

	ans := make([][]int, len(hull))
	for i, idx := range hull {
		ans[i] = trees[idx]
	}
	return ans
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(outerTrees([][]int{{1, 1}, {2, 2}, {2, 0}, {2, 4}, {3, 3}, {4, 2}}), [][]int{{1, 1}, {2, 0}, {4, 2}, {3, 3}, {2, 4}})
	assert(outerTrees([][]int{{1, 2}, {2, 2}, {4, 2}}), [][]int{{1, 2}, {2, 2}, {4, 2}})
}
