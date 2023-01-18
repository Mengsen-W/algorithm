/*
 * @Date: 2021-08-18 08:36:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-18 08:41:48
 */

package main

const MOD int = 1e9 + 7

type matrix [6][6]int

func (a matrix) mul(b matrix) matrix {
	c := matrix{}
	for i, row := range a {
		for j := range b[0] {
			for k, v := range row {
				c[i][j] = (c[i][j] + v*b[k][j]) % MOD
			}
		}
	}
	return c
}

func (a matrix) pow(n int) matrix {
	res := matrix{}
	for i := range res {
		res[i][i] = 1
	}
	for ; n > 0; n >>= 1 {
		if n&1 > 0 {
			res = res.mul(a)
		}
		a = a.mul(a)
	}
	return res
}

func checkRecord(n int) (ans int) {
	m := matrix{
		{1, 1, 0, 1, 0, 0},
		{1, 0, 1, 1, 0, 0},
		{1, 0, 0, 1, 0, 0},
		{0, 0, 0, 1, 1, 0},
		{0, 0, 0, 1, 0, 1},
		{0, 0, 0, 1, 0, 0},
	}
	res := m.pow(n)
	for _, v := range res[0] {
		ans = (ans + v) % MOD
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(checkRecord(2) == 8)
	assert(checkRecord(1) == 3)
	assert(checkRecord(10101) == 183236316)
}
