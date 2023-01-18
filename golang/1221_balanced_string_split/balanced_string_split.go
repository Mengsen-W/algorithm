/*
 * @Date: 2021-09-07 16:57:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-07 17:10:59
 */

package main

func balancedStringSplit(s string) (ans int) {
	d := 0
	for _, ch := range s {
		if ch == 'L' {
			d++
		} else {
			d--
		}
		if d == 0 {
			ans++
		}
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
		s := "RLRRLLRLRL"
		ans := 4
		assert(balancedStringSplit(s) == ans)
	}
	{
		s := "RLLLLRRRLR"
		ans := 3
		assert(balancedStringSplit(s) == ans)
	}
	{
		s := "LLLLRRRR"
		ans := 1
		assert(balancedStringSplit(s) == ans)
	}
	{
		s := "RLRRRLLRLL"
		ans := 2
		assert(balancedStringSplit(s) == ans)
	}
}
