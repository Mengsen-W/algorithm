/*
 * @Date: 2022-10-18
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-18
 * @FilePath: /algorithm/902_at_most_n_given_digit_set/at_most_n_given_digit_set.go
 */

package main

import "strconv"

func atMostNGivenDigitSet(digits []string, n int) (ans int) {
	m := len(digits)
	s := strconv.Itoa(n)
	bits := []int{}
	isLimit := true
	for _, c := range s {
		if !isLimit {
			bits = append(bits, m-1)
			continue
		}
		selectIndex := -1
		for j, d := range digits {
			if d[0] > byte(c) {
				break
			}
			selectIndex = j
		}
		if selectIndex >= 0 {
			bits = append(bits, selectIndex)
			if digits[selectIndex][0] < byte(c) {
				isLimit = false
			}
		} else {
			sz := len(bits)
			for len(bits) > 0 && bits[len(bits)-1] == 0 {
				bits = bits[:len(bits)-1]
			}
			if len(bits) > 0 {
				bits[len(bits)-1]--
			} else {
				sz--
			}
			for len(bits) <= sz {
				bits = append(bits, m-1)
			}
			isLimit = false
		}
	}
	for _, b := range bits {
		ans = ans*m + b + 1
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		digits := []string{"1", "3", "5", "7"}
		n := 100
		ans := 20
		assert(atMostNGivenDigitSet(digits, n) == ans)
	}

	{
		digits := []string{"1", "4", "9"}
		n := 1000000000
		ans := 29523
		assert(atMostNGivenDigitSet(digits, n) == ans)
	}

	{
		digits := []string{"7"}
		n := 8
		ans := 1
		assert(atMostNGivenDigitSet(digits, n) == ans)
	}
}
