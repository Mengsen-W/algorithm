/*
 * @Date: 2022-01-09 01:04:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-09 01:22:49
 */

package main

func slowestKey(releaseTimes []int, keysPressed string) byte {
	ans := keysPressed[0]
	maxTime := releaseTimes[0]
	for i := 1; i < len(keysPressed); i++ {
		key := keysPressed[i]
		time := releaseTimes[i] - releaseTimes[i-1]
		if time > maxTime || time == maxTime && key > ans {
			ans = key
			maxTime = time
		}
	}
	return ans
}

func main() {
	assert := func(a, b byte) {
		if a != b {
			panic("Not Passed")
		}
	}
	assert(slowestKey([]int{9, 29, 49, 50}, "cbcd"), 'c')
	assert(slowestKey([]int{12, 23, 36, 46, 62}, "spuda"), 'a')
}
