/*
 * @Date: 2022-10-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-30
 * @FilePath: /algorithm/784_letter_case_permutation/letter_case_permutation.go
 */

package main

import (
	"reflect"
	"unicode"
)

func letterCasePermutation(s string) (ans []string) {
	q := []string{""}
	for len(q) > 0 {
		cur := q[0]
		pos := len(cur)
		if pos == len(s) {
			ans = append(ans, cur)
			q = q[1:]
		} else {
			if unicode.IsLetter(rune(s[pos])) {
				q = append(q, cur+string(s[pos]^32))
			}
			q[0] += string(s[pos])
		}
	}
	return
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		s := "a1b2"
		ans := []string{"a1b2", "a1B2", "A1b2", "A1B2"}
		assert(letterCasePermutation(s), ans)
	}

	{
		s := "3z4"
		ans := []string{"3z4", "3Z4"}
		assert(letterCasePermutation(s), ans)
	}
}
