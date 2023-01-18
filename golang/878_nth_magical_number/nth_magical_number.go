/*
 * @Date: 2022-11-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-22
 * @FilePath: /algorithm/878_nth_magical_number/nth_magical_number.go
 */

package main

const mod int = 1e9 + 7

func nthMagicalNumber(n, a, b int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	var gcd func(a, b int) int
	gcd = func(a, b int) int {
		if b != 0 {
			return gcd(b, a%b)
		}
		return a
	}

	c := a / gcd(a, b) * b
	m := c/a + c/b - 1
	r := n % m
	res := c * (n / m) % mod
	if r == 0 {
		return res
	}
	addA := a
	addB := b
	for i := 0; i < r-1; i++ {
		if addA < addB {
			addA += a
		} else {
			addB += b
		}
	}
	return (res + min(addA, addB)%mod) % mod
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	// assert(nthMagicalNumber(1, 2, 3) == 2)
	assert(nthMagicalNumber(4, 2, 3) == 6)
}
