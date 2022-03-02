/*
 * @Date: 2022-03-02 00:10:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-02 01:29:08
 * @FilePath: /algorithm/564_nearest_palindromic/nearest_palindromic.go
 */

package main

import (
	"math"
	"strconv"
)

func nearestPalindromic(n string) string {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	m := len(n)
	candidates := []int{int(math.Pow10(m-1)) - 1, int(math.Pow10(m)) + 1}
	selfPrefix, _ := strconv.Atoi(n[:(m+1)/2])
	for _, x := range []int{selfPrefix - 1, selfPrefix, selfPrefix + 1} {
		y := x
		if m&1 == 1 {
			y /= 10
		}
		for ; y > 0; y /= 10 {
			x = x*10 + y%10
		}
		candidates = append(candidates, x)
	}

	ans := -1
	selfNumber, _ := strconv.Atoi(n)
	for _, candidate := range candidates {
		if candidate != selfNumber {
			if ans == -1 ||
				abs(candidate-selfNumber) < abs(ans-selfNumber) ||
				abs(candidate-selfNumber) == abs(ans-selfNumber) && candidate < ans {
				ans = candidate
			}
		}
	}
	return strconv.Itoa(ans)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(nearestPalindromic("123") == "121")
	assert(nearestPalindromic("1") == "0")
}
