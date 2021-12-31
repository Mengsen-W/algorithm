/*
 * @Date: 2021-12-31 01:05:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-31 01:15:31
 */

package main

func checkPerfectNumber(num int) bool {
	if num == 1 {
		return false
	}

	sum := 1
	for d := 2; d*d <= num; d++ {
		if num%d == 0 {
			sum += d
			if d*d < num {
				sum += num / d
			}
		}
	}
	return sum == num
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(checkPerfectNumber(28) == true)
	assert(checkPerfectNumber(6) == true)
	assert(checkPerfectNumber(496) == true)
	assert(checkPerfectNumber(8128) == true)
	assert(checkPerfectNumber(2) == false)
}
