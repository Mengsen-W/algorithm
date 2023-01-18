/*
 * @Date: 2022-01-31 02:47:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-31 03:09:34
 */

package main

func numberOfSteps(num int) (ans int) {
	for ; num > 0; num >>= 1 {
		ans += num & 1
		if num > 1 {
			ans++
		}
	}
	return
}

func bitsLen(x uint) int {
	clz := 0
	if x>>16 == 0 {
		clz += 16
		x <<= 16
	}
	if x>>24 == 0 {
		clz += 8
		x <<= 8
	}
	if x>>28 == 0 {
		clz += 4
		x <<= 4
	}
	if x>>30 == 0 {
		clz += 2
		x <<= 2
	}
	if x>>31 == 0 {
		clz++
	}
	return 32 - clz
}

func onesCount(num uint) int {
	num = num&0x55555555 + num>>1&0x55555555
	num = num&0x33333333 + num>>2&0x33333333
	num = num&0x0F0F0F0F + num>>4&0x0F0F0F0F
	num = num&0x00FF00FF + num>>8&0x00FF00FF
	num = num&0x0000FFFF + num>>16&0x0000FFFF
	return int(num)
}

func numberOfSteps2(num int) (ans int) {
	if num == 0 {
		return 0
	}
	return bitsLen(uint(num)) - 1 + onesCount(uint(num))
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(numberOfSteps(14) == 6)
	assert(numberOfSteps(8) == 4)
	assert(numberOfSteps(123) == 12)
	assert(numberOfSteps2(14) == 6)
	assert(numberOfSteps2(8) == 4)
	assert(numberOfSteps2(123) == 12)
}
