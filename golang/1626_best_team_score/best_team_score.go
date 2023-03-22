/*
 * @Date: 2023-03-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-22
 * @FilePath: /algorithm/golang/1626_best_team_score/best_team_score.go
 */

// Package main ...
package main

import "sort"

func bestTeamScore(scores []int, ages []int) int {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	n := len(scores)
	people := make([][]int, n)
	for i := range scores {
		people[i] = []int{scores[i], ages[i]}
	}
	sort.Slice(people, func(i, j int) bool {
		if people[i][0] < people[j][0] {
			return true
		} else if people[i][0] > people[j][0] {
			return false
		}
		return people[i][1] < people[j][1]
	})
	dp := make([]int, n)
	res := 0
	for i := 0; i < n; i++ {
		for j := 0; j < i; j++ {
			if people[j][1] <= people[i][1] {
				dp[i] = max(dp[i], dp[j])
			}
		}
		dp[i] += people[i][0]
		res = max(res, dp[i])
	}
	return res
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		scores := []int{1, 3, 5, 10, 15}
		ages := []int{1, 2, 3, 4, 5}
		ans := 34
		assert(bestTeamScore(scores, ages) == ans)
	}

	{
		scores := []int{4, 5, 6, 5}
		ages := []int{2, 1, 2, 1}
		ans := 16
		assert(bestTeamScore(scores, ages) == ans)
	}

	{
		scores := []int{1, 2, 3, 5}
		ages := []int{8, 9, 10, 1}
		ans := 6
		assert(bestTeamScore(scores, ages) == ans)
	}
}
