/*
 * @Date: 2023-04-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-06
 * @FilePath: /algorithm/golang/1017_base_neg2/base_neg2.go
 */

// Package main ...
package main

func baseNeg2(n int) string {
	val := 0x55555555 ^ (0x55555555 - n)
	if val == 0 {
		return "0"
	}
	res := []byte{}
	for val > 0 {
		res = append(res, '0'+byte(val&1))
		val >>= 1
	}
	for i, n := 0, len(res); i < n/2; i++ {
		res[i], res[n-1-i] = res[n-1-i], res[i]
	}
	return string(res)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		n := 2
		ans := "110"
		assert(baseNeg2(n) == ans)
	}

	{
		n := 3
		ans := "111"
		assert(baseNeg2(n) == ans)
	}

	{
		n := 4
		ans := "100"
		assert(baseNeg2(n) == ans)
	}
}
