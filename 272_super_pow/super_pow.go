/*
 * @Date: 2021-12-05 07:19:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-05 07:26:37
 */

package main

const mod = 1337

func pow(x, n int) int {
	res := 1
	for ; n > 0; n /= 2 {
		if n&1 > 0 {
			res = res * x % mod
		}
		x = x * x % mod
	}
	return res
}

func superPow(a int, b []int) int {
	ans := 1
	for _, e := range b {
		ans = pow(ans, 10) * pow(a, e) % mod
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(superPow(2, []int{3}) == 8)
	assert(superPow(2, []int{1, 0}) == 1024)
	assert(superPow(1, []int{4, 3, 3, 8, 5, 2}) == 1)
	assert(superPow(2147483647, []int{2, 0, 0}) == 1198)
}
