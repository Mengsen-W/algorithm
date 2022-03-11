/*
 * @Date: 2022-03-10 23:48:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-11 00:42:28
 * @FilePath: /algorithm/2049_count_highest_score_nodes/count_highest_score_nodes.go
 */

package main

func countHighestScoreNodes(parents []int) (ans int) {
	n := len(parents)
	children := make([][]int, n)
	for node := 1; node < n; node++ {
		p := parents[node]
		children[p] = append(children[p], node)
	}

	maxScore := 0
	var dfs func(int) int
	dfs = func(node int) int {
		score, size := 1, n-1
		for _, ch := range children[node] {
			sz := dfs(ch)
			score *= sz
			size -= sz
		}
		if node > 0 {
			score *= size
		}
		if score == maxScore {
			ans++
		} else if score > maxScore {
			maxScore = score
			ans = 1
		}
		return n - size
	}
	dfs(0)
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(countHighestScoreNodes([]int{-1, 2, 0, 2, 0}) == 3)
	assert(countHighestScoreNodes([]int{-1, 2, 0}) == 2)
}
