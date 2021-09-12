/*
 * @Date: 2021-09-12 08:20:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-12 08:31:05
 */

package main

func checkValidString(s string) bool {
	minCount, maxCount := 0, 0
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	for _, ch := range s {
		if ch == '(' {
			minCount++
			maxCount++
		} else if ch == ')' {
			minCount = max(minCount-1, 0)
			maxCount--
			if maxCount < 0 {
				return false
			}
		} else {
			minCount = max(minCount-1, 0)
			maxCount++
		}
	}
	return minCount == 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		s := "()"
		assert(checkValidString(s))
	}
	{
		s := "(*)"
		assert(checkValidString(s))
	}
	{
		s := "(*))"
		assert(checkValidString(s))
	}
}
