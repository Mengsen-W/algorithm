/*
 * @Date: 2022-01-03 01:10:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-03 01:45:41
 * @FilePath: /algorithm/1185_day_of_the_week/day_of_the_week.go
 * @Description: file content
 */

package main

var week = []string{"Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"}
var monthDays = []int{31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30}

func dayOfTheWeek(day, month, year int) string {
	days := 0
	// 输入年份之前的年份的天数贡献
	days += 365*(year-1971) + (year-1969)/4
	// 输入年份中，输入月份之前的月份的天数贡献
	for _, d := range monthDays[:month-1] {
		days += d
	}
	if month >= 3 && (year%400 == 0 || year%4 == 0 && year%100 != 0) {
		days++
	}
	// 输入月份中的天数贡献
	days += day
	return week[(days+3)%7]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(dayOfTheWeek(31, 8, 2019) == "Saturday")
	assert(dayOfTheWeek(18, 7, 1999) == "Sunday")
	assert(dayOfTheWeek(15, 8, 1993) == "Sunday")
	assert(dayOfTheWeek(29, 2, 2016) == "Monday")
}
