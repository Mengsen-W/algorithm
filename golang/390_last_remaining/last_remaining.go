/*
 * @Date: 2022-01-02 01:34:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-02 01:37:15
 */

package main

func lastRemaining(n int) int {
	a1, an := 1, n
	k, cnt, step := 0, n, 1
	for cnt > 1 {
		if k%2 == 0 { // 正向
			a1 += step
			if cnt%2 == 1 {
				an -= step
			}
		} else { // 反向
			if cnt%2 == 1 {
				a1 += step
			}
			an -= step
		}
		k++
		cnt >>= 1
		step <<= 1
	}
	return a1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(lastRemaining(9) == 6)
	assert(lastRemaining(1) == 1)
}
