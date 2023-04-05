/*
 * @Date: 2023-04-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-05
 * @FilePath: /algorithm/golang/2427_common_factors/common_factors.go
 */

// Package main ...
package main

func commonFactors(a int, b int) int {
	gcd := func(a, b int) int {
		for a != 0 {
			a, b = b%a, a
		}
		return b
	}
	g := gcd(a, b)
	ans := 0
	for i := 1; i*i <= g; i++ {
		if g%i == 0 {
			ans++
			if i*i < g {
				ans++
			}
		}
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		a := 12
		b := 6
		ans := 4
		assert(commonFactors(a, b) == ans)
	}

	{
		a := 25
		b := 30
		ans := 2
		assert(commonFactors(a, b) == ans)
	}
}
