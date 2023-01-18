/*
 * @Date: 2022-12-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-09
 * @FilePath: /algorithm/1780_check_powers_of_three/check_powers_of_three.go
 */

package main

func checkPowersOfThree(n int) bool {
	for ; n > 0; n /= 3 {
		if n%3 == 2 {
			return false
		}
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(checkPowersOfThree(12))
	assert(checkPowersOfThree(91))
	assert(!checkPowersOfThree(21))
}
