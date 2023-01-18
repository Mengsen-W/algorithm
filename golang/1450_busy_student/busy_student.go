/*
 * @Date: 2022-08-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-19
 * @FilePath: /algorithm/1450_busy_student/busy_student.go
 */

package main

import "sort"

func busyStudent(startTime []int, endTime []int, queryTime int) (ans int) {
	sort.Ints(startTime)
	sort.Ints(endTime)
	return sort.SearchInts(startTime, queryTime+1) - sort.SearchInts(endTime, queryTime)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		startTime := []int{1, 2, 3}
		endTime := []int{3, 2, 7}
		queryTime := 4
		assert(busyStudent(startTime, endTime, queryTime) == 1)
	}

	{
		startTime := []int{4}
		endTime := []int{4}
		queryTime := 4
		assert(busyStudent(startTime, endTime, queryTime) == 1)
	}

	{
		startTime := []int{1, 1, 1, 1}
		endTime := []int{1, 3, 2, 4}
		queryTime := 7
		assert(busyStudent(startTime, endTime, queryTime) == 0)
	}

	{
		startTime := []int{9, 8, 7, 6, 5, 4, 3, 2, 1}
		endTime := []int{10, 10, 10, 10, 10, 10, 10, 10, 10}
		queryTime := 5
		assert(busyStudent(startTime, endTime, queryTime) == 5)
	}

	{
		startTime := []int{1, 2, 3}
		endTime := []int{3, 2, 7}
		queryTime := 4
		assert(busyStudent(startTime, endTime, queryTime) == 1)
	}
}
