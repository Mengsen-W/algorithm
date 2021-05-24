/*
 * @Date: 2021-05-24 10:20:09
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-24 10:25:21
 */

package main

func xorGame(nums []int) bool {
	if len(nums)%2 == 0 {
		return true
	}
	xor := 0
	for _, num := range nums {
		xor ^= num
	}
	return xor == 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(!xorGame([]int{1, 1, 2}))
}
