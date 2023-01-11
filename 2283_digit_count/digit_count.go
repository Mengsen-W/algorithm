/*
 * @Date: 2023-01-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-11
 * @FilePath: /algorithm/2283_digit_count/digit_count.go
 */

package main

func digitCount(num string) bool {
	cnt := map[rune]int{}
	for _, c := range num {
		cnt[c-'0']++
	}
	for i, c := range num {
		if cnt[rune(i)] != int(c-'0') {
			return false
		}
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		num := "1210"
		ans := true
		assert(digitCount(num) == ans)
	}

	{
		num := "030"
		ans := false
		assert(digitCount(num) == ans)
	}
}
