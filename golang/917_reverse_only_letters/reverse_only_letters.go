/*
 * @Date: 2022-02-23 12:53:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-23 12:59:12
 * @FilePath: /algorithm/917_reverse_only_letters/reverse_only_letters.go
 */

package main

import (
	"reflect"
	"unicode"
)

func reverseOnlyLetters(s string) string {
	ans := []byte(s)
	left, right := 0, len(s)-1
	for {
		for left < right && !unicode.IsLetter(rune(s[left])) {
			left++
		}
		for right > left && !unicode.IsLetter(rune(s[right])) {
			right--
		}
		if left >= right {
			break
		}
		ans[left], ans[right] = ans[right], ans[left]
		left++
		right--
	}
	return string(ans)
}

func main() {
	assert := func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("assert failed")
		}
	}

	assert(reverseOnlyLetters("ab-cd"), "dc-ba")
	assert(reverseOnlyLetters("a-bC-dEf-ghIj"), "j-Ih-gfE-dCba")
	assert(reverseOnlyLetters("Test1ng-Leet=code-Q!"), "Qedo1ct-eeLg=ntse-T!")
}
