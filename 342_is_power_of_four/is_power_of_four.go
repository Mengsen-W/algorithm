/*
 * @Date: 2021-05-31 09:05:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-31 09:12:02
 * @FilePath: \algorithm\342_is_power_of_four\is_power_of_four.go
 * @Description: file content
 */

package main

func isPowerOfFour(n int) bool {
	return n > 0 && n&(n-1) == 0 && n%3 == 1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(isPowerOfFour(16))
	assert(!isPowerOfFour(5))
	assert(isPowerOfFour(1))

}
