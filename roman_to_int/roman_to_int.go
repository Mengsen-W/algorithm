/*
 * @Date: 2021-05-15 14:06:41
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-15 14:13:17
 */

package main

var symbolValues = map[byte]int{'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}

func romanToInt(s string) (ans int) {
	n := len(s)
	for i := range s {
		value := symbolValues[s[i]]
		if i < n-1 && value < symbolValues[s[i+1]] {
			ans -= value
		} else {
			ans += value
		}
	}
	return
}

func assert(b bool) {
	if !b {
		panic("Not Passed!")
	}
}

func main() {
	assert(romanToInt("III") == 3)
	assert(romanToInt("IV") == 4)
	assert(romanToInt("IX") == 9)
	assert(romanToInt("LVIII") == 58)
	assert(romanToInt("MCMXCIV") == 1994)
}
