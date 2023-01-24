/*
 * @Date: 2023-01-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-24
 * @FilePath: /algorithm/golang/1828_count_points/count_points.go
 */

package main

import "reflect"

func countPoints(points [][]int, queries [][]int) []int {
	ans := make([]int, len(queries))
	for i, q := range queries {
		x, y, r := q[0], q[1], q[2]
		for _, p := range points {
			if (p[0]-x)*(p[0]-x)+(p[1]-y)*(p[1]-y) <= r*r {
				ans[i]++
			}
		}
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		points := [][]int{{1, 3}, {3, 3}, {5, 3}, {2, 2}}
		queries := [][]int{{2, 3, 1}, {4, 3, 1}, {1, 1, 2}}
		ans := []int{3, 2, 2}
		assert(countPoints(points, queries), ans)
	}

	{
		points := [][]int{{1, 1}, {2, 2}, {3, 3}, {4, 4}, {5, 5}}
		queries := [][]int{{1, 2, 2}, {2, 2, 2}, {4, 3, 2}, {4, 3, 3}}
		ans := []int{2, 3, 2, 4}
		assert(countPoints(points, queries), ans)
	}
}
