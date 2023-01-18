/*
 * @Date: 2021-10-29 02:27:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-29 02:35:22
 */

package main

func isSelfCrossing(distance []int) bool {
	n := len(distance)

	// 处理第 1 种情况
	i := 0
	for i < n && (i < 2 || distance[i] > distance[i-2]) {
		i++
	}

	if i == n {
		return false
	}

	// 处理第 j 次移动的情况
	if i == 3 && distance[i] == distance[i-2] ||
		i >= 4 && distance[i] >= distance[i-2]-distance[i-4] {
		distance[i-1] -= distance[i-3]
	}
	i++

	// 处理第 2 种情况
	for i < n && distance[i] < distance[i-2] {
		i++
	}

	return i != n
}

func main() {
	assert := func(a bool) {
		if !a {
			panic("assert failed")
		}
	}
	assert(isSelfCrossing([]int{1, 1, 1, 1}) == true)
	assert(isSelfCrossing([]int{1, 2, 3, 4}) == false)
	assert(isSelfCrossing([]int{2, 1, 1, 2}) == true)
}
