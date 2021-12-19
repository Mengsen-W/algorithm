/*
 * @Date: 2021-12-19 01:00:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-19 01:18:44
 */

package main

func findJudge(n int, trust [][]int) int {
	inDegrees := make([]int, n+1)
	outDegrees := make([]int, n+1)
	for _, t := range trust {
		inDegrees[t[1]]++
		outDegrees[t[0]]++
	}
	for i := 1; i <= n; i++ {
		if inDegrees[i] == n-1 && outDegrees[i] == 0 {
			return i
		}
	}
	return -1
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}
	assert(findJudge(2, [][]int{{1, 2}}), 2)
	assert(findJudge(3, [][]int{{1, 3}, {2, 3}}), 3)
	assert(findJudge(3, [][]int{{1, 3}, {2, 3}, {3, 1}}), -1)
	assert(findJudge(3, [][]int{{1, 2}, {2, 3}}), -1)
	assert(findJudge(4, [][]int{{1, 3}, {1, 4}, {2, 3}, {2, 4}, {4, 3}}), 3)
}
