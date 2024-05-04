/*
 * @Date: 2022-10-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-04
 * @FilePath: /algorithm/golang/1235_job_scheduling/job_scheduling.go
 */

package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

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
	tests := []struct {
		startTime []int
		endTime   []int
		profit    []int
		ans       int
	}{
		{[]int{1, 2, 3, 3}, []int{3, 4, 5, 6}, []int{50, 10, 40, 70}, 120},
		{[]int{1, 2, 3, 4, 6}, []int{3, 5, 10, 6, 9}, []int{20, 20, 100, 70, 60}, 150},
		{[]int{1, 1, 1}, []int{2, 3, 4}, []int{5, 6, 4}, 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, jobScheduling(test.startTime, test.endTime, test.profit), index)
	}
}
