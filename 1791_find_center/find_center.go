/*
 * @Date: 2022-02-18 02:02:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-18 02:55:03
 */

package main

func findCenter1(edges [][]int) int {
	n := len(edges)
	nums := make(map[int]int)
	for _, v := range edges {
		nums[v[0]]++
		nums[v[1]]++
	}
	for k, v := range nums {
		if v == n {
			return k
		}
	}
	return 0
}

func findCenter2(edges [][]int) int {
	if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
		return edges[0][0]
	} else {
		return edges[0][1]
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(findCenter1([][]int{{1, 2}, {2, 3}, {4, 2}}) == 2)
	assert(findCenter2([][]int{{1, 2}, {2, 3}, {4, 2}}) == 2)
	assert(findCenter1([][]int{{1, 2}, {5, 1}, {1, 3}, {1, 4}}) == 1)
	assert(findCenter2([][]int{{1, 2}, {5, 1}, {1, 3}, {1, 4}}) == 1)
}
