/*
 * @Date: 2022-03-03 00:27:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-03 00:40:11
 * @FilePath: /algorithm/258_add_digits/add_digits.go
 */

package main

func addDigits1(num int) int {
	for num >= 10 {
		sum := 0
		for num > 0 {
			sum += num % 10
			num /= 10
		}
		num = sum
	}
	return num
}

func addDigits2(num int) int {
	return (num-1)%9 + 1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(addDigits1(38) == 2)
	assert(addDigits2(38) == 2)

	assert(addDigits1(0) == 0)
	assert(addDigits2(0) == 0)
}
