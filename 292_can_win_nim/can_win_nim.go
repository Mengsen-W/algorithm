/*
 * @Date: 2021-09-18 08:50:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-18 08:53:52
 */

package main

func canWinNim(n int) bool {
	return n%4 != 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(!canWinNim(4))
	assert(canWinNim(1))
	assert(canWinNim(2))
}
