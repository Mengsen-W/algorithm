/*
 * @Date: 2022-08-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-06
 * @FilePath: /algorithm/1408_string_matching/string_matching.go
 */

package main

import (
	"reflect"
	"strings"
)

func stringMatching(words []string) (ans []string) {
	ans = []string{}
	for i, x := range words {
		for j, y := range words {
			if j != i && strings.Contains(y, x) {
				ans = append(ans, x)
				break
			}
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
		words := []string{"mass", "as", "hero", "superhero"}
		ans := []string{"as", "hero"}
		assert(stringMatching(words), ans)
	}

	{
		words := []string{"leetcode", "et", "code"}
		ans := []string{"et", "code"}
		assert(stringMatching(words), ans)
	}

	{
		words := []string{"blue", "green", "bu"}
		ans := []string{}
		assert(stringMatching(words), ans)
	}
}
