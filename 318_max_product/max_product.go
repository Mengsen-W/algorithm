/*
 * @Date: 2021-11-16 23:38:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-16 23:46:59
 */

package main

func maxProduct(words []string) (ans int) {
	masks := map[int]int{}
	for _, word := range words {
		mask := 0
		for _, ch := range word {
			mask |= 1 << (ch - 'a')
		}
		if len(word) > masks[mask] {
			masks[mask] = len(word)
		}
	}

	for x, lenX := range masks {
		for y, lenY := range masks {
			if x&y == 0 && lenX*lenY > ans {
				ans = lenX * lenY
			}
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

	assert(maxProduct([]string{"abcw", "baz", "foo", "bar", "xtfn", "abcdef"}) == 16)
	assert(maxProduct([]string{"a", "ab", "abc", "d", "cd", "bcd", "abcd"}) == 4)
	assert(maxProduct([]string{"a", "aa", "aaa", "aaaa"}) == 0)
}
