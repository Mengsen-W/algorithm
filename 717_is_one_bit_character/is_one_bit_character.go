/*
 * @Date: 2022-02-20 00:38:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-20 01:09:13
 */

package main

func isOneBitCharacter(bits []int) bool {
	n := len(bits)
	i := n - 2
	for i >= 0 && bits[i] == 1 {
		i--
	}
	return (n-i)%2 == 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(isOneBitCharacter([]int{1, 0, 0}))
	assert(!isOneBitCharacter([]int{1, 1, 1, 0}))
}
