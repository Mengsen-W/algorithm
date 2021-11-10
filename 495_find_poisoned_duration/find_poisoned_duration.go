/*
 * @Date: 2021-11-10 00:52:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-10 01:16:24
 */

package main

func findPoisonedDuration(timeSeries []int, duration int) int {
	n := len(timeSeries)
	var sum, l, r int = 0, 0, 0
	for i := 0; i < n; i++ {
		if timeSeries[i] > r {
			sum += r - l
			l, r = timeSeries[i], timeSeries[i]+duration
		} else {
			r = timeSeries[i] + duration
		}
	}
	sum += r - l
	return sum
}

func main() {
	assert := func(a bool) {
		if !a {
			panic("assertion failed")
		}
	}
	assert(findPoisonedDuration([]int{1, 4}, 2) == 4)
	assert(findPoisonedDuration([]int{1, 2}, 2) == 3)
}
