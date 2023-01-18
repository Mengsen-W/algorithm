/*
 * @Date: 2022-04-11 10:28:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-11 10:32:45
 * @FilePath: /algorithm/357_count_numbers_with_unique_digits/count_numbers_with_unique_digits.go
 */

package main

func countNumbersWithUniqueDigits(n int) int {
	if n == 0 {
		return 1
	}
	if n == 1 {
		return 10
	}
	ans, cur := 10, 9
	for i := 0; i < n-1; i++ {
		cur *= 9 - i
		ans += cur
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("assert failed")
		}
	}

	assert(countNumbersWithUniqueDigits(2) == 91)
	assert(countNumbersWithUniqueDigits(0) == 1)
}
