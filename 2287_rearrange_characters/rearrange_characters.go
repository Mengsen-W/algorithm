/*
 * @Date: 2023-01-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-13
 * @FilePath: /algorithm/2287_rearrange_characters/rearrange_characters.go
 */

package main

func rearrangeCharacters(s, target string) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	var cntS, cntT [26]int
	for _, c := range s {
		cntS[c-'a']++
	}
	for _, c := range target {
		cntT[c-'a']++
	}
	ans := len(s)
	for i, c := range cntT {
		if c > 0 {
			ans = min(ans, cntS[i]/c)
			if ans == 0 {
				return 0
			}
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
		s := "ilovecodingonleetcode"
		target := "code"
		ans := 2
		assert(rearrangeCharacters(s, target) == ans)
	}

	{
		s := "abcba"
		target := "abc"
		ans := 1
		assert(rearrangeCharacters(s, target) == ans)
	}

	{
		s := "abbaccaddaeea"
		target := "aaaaa"
		ans := 1
		assert(rearrangeCharacters(s, target) == ans)
	}
}
