/*
 * @Date: 2022-10-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-22
 * @FilePath: /algorithm/1235_job_scheduling/job_scheduling.go
 */

package main

import "sort"

func jobScheduling(startTime, endTime, profit []int) int {
	n := len(startTime)
	jobs := make([][3]int, n)
	for i := 0; i < n; i++ {
		jobs[i] = [3]int{startTime[i], endTime[i], profit[i]}
	}
	sort.Slice(jobs, func(i, j int) bool { return jobs[i][1] < jobs[j][1] })

	dp := make([]int, n+1)
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	for i := 1; i <= n; i++ {
		k := sort.Search(i, func(j int) bool { return jobs[j][1] > jobs[i-1][0] })
		dp[i] = max(dp[i-1], dp[k]+jobs[i-1][2])
	}
	return dp[n]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		startTime := []int{1, 2, 3, 3}
		endTime := []int{3, 4, 5, 6}
		profit := []int{50, 10, 40, 70}
		ans := 120
		assert(jobScheduling(startTime, endTime, profit) == ans)
	}

	{
		startTime := []int{1, 2, 3, 4, 6}
		endTime := []int{3, 5, 10, 6, 9}
		profit := []int{20, 20, 100, 70, 60}
		ans := 150
		assert(jobScheduling(startTime, endTime, profit) == ans)
	}

	{
		startTime := []int{1, 1, 1}
		endTime := []int{2, 3, 4}
		profit := []int{5, 6, 4}
		ans := 6
		assert(jobScheduling(startTime, endTime, profit) == ans)
	}
}
