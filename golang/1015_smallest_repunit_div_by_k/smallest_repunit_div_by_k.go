/*
 * @Date: 2023-05-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-10
 * @FilePath: /algorithm/golang/1015_smallest_repunit_div_by_k/smallest_repunit_div_by_k.go
 */

// Package main ...
package main

func smallestRepunitDivByK(k int) int {
	// 如果 k 是偶数或者以 5 结尾，那么一定不存在这样的正整数，直接返回 -1。
	if k%2 == 0 || k%5 == 0 {
		return -1
	}
	// cur 代表当前的余数，初始值为 0，res 代表数字 1 的个数，初始值为 1。
	cur, res := 0, 1

	// 不断计算下一个数字的余数，直到找到一个可行的解或者发现不存在可行的解。
	for {
		cur = (10*cur + 1) % k
		if cur == 0 {
			return res
		}
		res++
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(smallestRepunitDivByK(1) == 1)
	assert(smallestRepunitDivByK(2) == -1)
	assert(smallestRepunitDivByK(3) == 3)
}
