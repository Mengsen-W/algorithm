// Package main ...
package main

func countDigits(n int) (cnt [10]int) {
	for n > 0 {
		cnt[n%10]++
		n /= 10
	}
	return
}

var powerOf2Digits = map[[10]int]bool{}

func init() {
	for n := 1; n <= 1e9; n <<= 1 {
		powerOf2Digits[countDigits(n)] = true
	}
}

func reorderedPowerOf2(n int) bool {
	return powerOf2Digits[countDigits(n)]
}

func main() {
	assert := func(result bool) {
		if !result {
			panic("assert failed")
		}
	}
	assert(reorderedPowerOf2(1) == true)
	assert(reorderedPowerOf2(10) == false)
	assert(reorderedPowerOf2(16) == true)
	assert(reorderedPowerOf2(24) == false)
	assert(reorderedPowerOf2(46) == true)
	assert(reorderedPowerOf2(8208) == false)
	assert(reorderedPowerOf2(9474) == false)
	assert(reorderedPowerOf2(15) == false)
	assert(reorderedPowerOf2(32) == true)
	assert(reorderedPowerOf2(8) == true)
	assert(reorderedPowerOf2(2147483647) == false)
	assert(reorderedPowerOf2(2147483648) == false)
}
