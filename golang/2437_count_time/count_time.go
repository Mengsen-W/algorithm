/*
 * @Date: 2023-05-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-09
 * @FilePath: /algorithm/golang/2437_count_time/count_time.go
 */

// Package main ...
package main

func countTime(time string) int {
	countHour := 0
	countMinute := 0
	for i := 0; i < 24; i++ {
		hiHour := byte(i / 10)
		loHour := byte(i % 10)
		if (time[0] == '?' || time[0] == hiHour+'0') &&
			(time[1] == '?' || time[1] == loHour+'0') {
			countHour++
		}
	}
	for i := 0; i < 60; i++ {
		hiMinute := byte(i / 10)
		loMinute := byte(i % 10)
		if (time[3] == '?' || time[3] == hiMinute+'0') &&
			(time[4] == '?' || time[4] == loMinute+'0') {
			countMinute++
		}
	}
	return countHour * countMinute
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		time := "?5:00"
		ans := 2
		assert(countTime(time) == ans)
	}

	{
		time := "0?:0?"
		ans := 100
		assert(countTime(time) == ans)
	}

	{
		time := "??:??"
		ans := 1440
		assert(countTime(time) == ans)
	}
}
