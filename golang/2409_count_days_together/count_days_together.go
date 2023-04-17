/*
 * @Date: 2023-04-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-17
 * @FilePath: /algorithm/golang/2409_count_days_together/count_days_together.go
 */

// Package main ...
package main

import "strconv"

func countDaysTogether(arriveAlice string, leaveAlice string, arriveBob string, leaveBob string) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	datesOfMonths := []int{31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}
	prefixSum := make([]int, 1)
	for _, date := range datesOfMonths {
		prefixSum = append(prefixSum, prefixSum[len(prefixSum)-1]+date)
	}
	calculateDayOfYear := func(day string, prefixSum []int) int {
		month, _ := strconv.Atoi(day[:2])
		date, _ := strconv.Atoi(day[3:])
		return prefixSum[month-1] + date
	}

	arriveAliceDay := calculateDayOfYear(arriveAlice, prefixSum)
	leaveAliceDay := calculateDayOfYear(leaveAlice, prefixSum)
	arriveBobDay := calculateDayOfYear(arriveBob, prefixSum)
	leaveBobDay := calculateDayOfYear(leaveBob, prefixSum)

	return max(0, min(leaveAliceDay, leaveBobDay)-max(arriveAliceDay, arriveBobDay)+1)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		arriveAlice := "08-15"
		leaveAlice := "08-18"
		arriveBob := "08-16"
		leaveBob := "08-19"
		ans := 3
		assert(countDaysTogether(arriveAlice, leaveAlice, arriveBob, leaveBob) == ans)
	}

	{
		arriveAlice := "10-01"
		leaveAlice := "10-31"
		arriveBob := "11-01"
		leaveBob := "12-31"
		ans := 0
		assert(countDaysTogether(arriveAlice, leaveAlice, arriveBob, leaveBob) == ans)
	}
}
