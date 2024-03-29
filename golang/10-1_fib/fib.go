/*
 * @Date: 2021-09-04 10:14:41
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-04 10:17:01
 */

package main

const mod int = 1e9 + 7

type matrix [2][2]int

func multiply(a, b matrix) (c matrix) {
	for i := 0; i < 2; i++ {
		for j := 0; j < 2; j++ {
			c[i][j] = (a[i][0]*b[0][j] + a[i][1]*b[1][j]) % mod
		}
	}
	return
}

func pow(a matrix, n int) matrix {
	ret := matrix{{1, 0}, {0, 1}}
	for ; n > 0; n >>= 1 {
		if n&1 == 1 {
			ret = multiply(ret, a)
		}
		a = multiply(a, a)
	}
	return ret
}

func fib(n int) int {
	if n < 2 {
		return n
	}
	res := pow(matrix{{1, 1}, {1, 0}}, n-1)
	return res[0][0]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(fib(2) == 1)
	assert(fib(5) == 5)
}
