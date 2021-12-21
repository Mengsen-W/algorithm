/*
 * @Date: 2021-12-21 01:20:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-21 01:49:25
 */

package main

import "strconv"

func dayOfYear(date string) int {
	year, _ := strconv.Atoi(date[:4])
	month, _ := strconv.Atoi(date[5:7])
	day, _ := strconv.Atoi(date[8:])

	days := []int{31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}
	if year%400 == 0 || (year%4 == 0 && year%100 != 0) {
		days[1]++
	}

	ans := day
	for _, d := range days[:month-1] {
		ans += d
	}
	return ans
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("assert failed")
		}
	}

	assert(dayOfYear("2019-01-09"), 9)
	assert(dayOfYear("2019-02-10"), 41)
	assert(dayOfYear("2003-03-01"), 60)
	assert(dayOfYear("2004-03-01"), 61)
}
