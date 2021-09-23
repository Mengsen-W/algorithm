/*
 * @Date: 2021-09-21 09:05:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-21 09:37:28
 */

package main

func lengthOfLastWord(s string) int {
	var ans int
	for i := len(s) - 1; i >= 0; i-- {
		if s[i] == ' ' && ans > 0 {
			break
		}
		if s[i] != ' ' {
			ans++
		}
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		s := "Hello World"
		assert(lengthOfLastWord(s) == 5)
	}
	{
		s := "   fly me   to   the moon  "
		assert(lengthOfLastWord(s) == 4)
	}
	{
		s := "luffy is still joyboy"
		assert(lengthOfLastWord(s) == 6)
	}
}
