/*
 * @Date: 2023-04-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-08
 * @FilePath: /algorithm/golang/1125_smallest_sufficient_team/smallest_sufficient_team.go
 */

// Package main ...
package main

import "reflect"

func smallestSufficientTeam(req_skills []string, people [][]string) []int {
	n, m := len(req_skills), len(people)
	skill_index := make(map[string]int)
	for i, skill := range req_skills {
		skill_index[skill] = i
	}
	dp := make([]int, 1<<n)
	for i := range dp {
		dp[i] = m
	}
	dp[0] = 0
	prev_skill := make([]int, 1<<n)
	prev_people := make([]int, 1<<n)
	for i := 0; i < m; i++ {
		cur_skill := 0
		for _, s := range people[i] {
			cur_skill |= 1 << skill_index[s]
		}
		for prev := 0; prev < (1 << n); prev++ {
			comb := prev | cur_skill
			if dp[comb] > dp[prev]+1 {
				dp[comb] = dp[prev] + 1
				prev_skill[comb] = prev
				prev_people[comb] = i
			}
		}
	}
	res := []int{}
	i := (1 << n) - 1
	for i > 0 {
		res = append(res, prev_people[i])
		i = prev_skill[i]
	}
	return res
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		req_skills := []string{"java", "nodejs", "reactjs"}
		people := [][]string{{"java"}, {"nodejs"}, {"nodejs", "reactjs"}}
		ans := []int{2, 0}
		assert(smallestSufficientTeam(req_skills, people), ans)
	}

	{
		req_skills := []string{"algorithms", "math", "java", "reactjs", "csharp", "aws"}
		people := [][]string{{"algorithms", "math", "java"},
			{"algorithms", "math", "reactjs"},
			{"java", "csharp", "aws"},
			{"reactjs", "csharp"},
			{"csharp", "math"},
			{"aws", "java"}}
		ans := []int{2, 1}
		assert(smallestSufficientTeam(req_skills, people), ans)
	}
}
