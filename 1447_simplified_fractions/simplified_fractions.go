/*
 * @Date: 2022-02-10 00:35:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-10 00:49:05
 */

package main

import (
	"reflect"
	"strconv"
)

func simplifiedFractions(n int) (ans []string) {
	for denominator := 2; denominator <= n; denominator++ {
		for numerator := 1; numerator < denominator; numerator++ {
			if gcd(numerator, denominator) == 1 {
				ans = append(ans, strconv.Itoa(numerator)+"/"+strconv.Itoa(denominator))
			}
		}
	}
	return
}

func gcd(a, b int) int {
	for a != 0 {
		a, b = b%a, a
	}
	return b
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(simplifiedFractions(2), []string{"1/2"})
	assert(simplifiedFractions(3), []string{"1/2", "1/3", "2/3"})
	assert(simplifiedFractions(4), []string{"1/2", "1/3", "2/3", "1/4", "3/4"})
	assert(simplifiedFractions(1), nil)
}
