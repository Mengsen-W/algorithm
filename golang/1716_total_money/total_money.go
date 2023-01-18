/*
 * @Date: 2022-01-15 01:43:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-15 02:17:01
 */

package main

func totalMoney(n int) (ans int) {
	// 所有完整的周存的钱
	weekNum := n / 7
	firstWeekMoney := (1 + 7) * 7 / 2
	lastWeekMoney := firstWeekMoney + 7*(weekNum-1)
	weekMoney := (firstWeekMoney + lastWeekMoney) * weekNum / 2
	// 剩下的不能构成一个完整的周的天数里存的钱
	dayNum := n % 7
	firstDayMoney := 1 + weekNum
	lastDayMoney := firstDayMoney + dayNum - 1
	dayMoney := (firstDayMoney + lastDayMoney) * dayNum / 2
	return weekMoney + dayMoney
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}

	assert(totalMoney(4), 10)
	assert(totalMoney(10), 37)
	assert(totalMoney(20), 96)
}
