/*
 * @Date: 2022-11-13
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-13
 * @FilePath: /algorithm/791_custom_sort_string/custom_sort_string.go
 */

package main

import (
	"fmt"
	"strings"
)

func customSortString(order, s string) string {
	freq := [26]int{}
	for _, c := range s {
		freq[c-'a']++
	}
	t := &strings.Builder{}
	for _, c := range order {
		if freq[c-'a'] > 0 {
			t.WriteString(strings.Repeat(string(c), freq[c-'a']))
			freq[c-'a'] = 0
		}
	}
	for i, k := range freq {
		if k > 0 {
			t.WriteString(strings.Repeat(fmt.Sprintf("%c", 'a'+i), k))
		}
	}
	return t.String()
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		order := "cba"
		s := "abcd"
		ans := "cbad"
		assert(customSortString(order, s) == ans)
	}

	{
		order := "cbafg"
		s := "abcd"
		ans := "cbad"
		assert(customSortString(order, s) == ans)
	}
}
