/*
 * @Date: 2021-10-07 18:44:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-07 18:45:04
 */

package main

func countSegments(s string) (ans int) {
	for i, ch := range s {
		if (i == 0 || s[i-1] == ' ') && ch != ' ' {
			ans++
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Paased")
		}
	}
	assert(countSegments("Hello, my name is John") == 5)
}
