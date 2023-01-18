/*
 * @Date: 2022-12-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-06
 * @FilePath: /algorithm/1805_num_different_integers/num_different_integers.go
 */

package main

import "unicode"

func numDifferentIntegers(word string) int {
	s := map[string]bool{}
	n := len(word)
	p1 := 0
	for {
		for p1 < n && !unicode.IsDigit(rune(word[p1])) {
			p1++
		}
		if p1 == n {
			break
		}
		p2 := p1
		for p2 < n && unicode.IsDigit(rune(word[p2])) {
			p2++
		}
		for p2-p1 > 1 && word[p1] == '0' { // 去除前导 0
			p1++
		}
		s[word[p1:p2]] = true
		p1 = p2
	}
	return len(s)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		word := "a123bc34d8ef34"
		ans := 3
		assert(numDifferentIntegers(word) == ans)
	}

	{
		word := "leet1234code234"
		ans := 2
		assert(numDifferentIntegers(word) == ans)
	}

	{
		word := "a1b01c001"
		ans := 1
		assert(numDifferentIntegers(word) == ans)
	}
}
