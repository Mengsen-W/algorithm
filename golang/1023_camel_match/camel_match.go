/*
 * @Date: 2023-04-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-14
 * @FilePath: /algorithm/golang/1023_camel_match/camel_match.go
 */

// Package main ...
package main

import (
	"reflect"
	"unicode"
)

func camelMatch(queries []string, pattern string) []bool {
	n := len(queries)
	res := make([]bool, n)
	for i := 0; i < n; i++ {
		res[i] = true
		p := 0
		for _, c := range queries[i] {
			if p < len(pattern) && pattern[p] == byte(c) {
				p++
			} else if unicode.IsUpper(c) {
				res[i] = false
				break
			}
		}
		if p < len(pattern) {
			res[i] = false
		}
	}
	return res
}

func main() {
	assert := func(a, b []bool) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		queries := []string{"FooBar", "FooBarTest", "FootBall", "FrameBuffer", "ForceFeedBack"}
		pattern := "FB"
		ans := []bool{true, false, true, true, false}
		assert(camelMatch(queries, pattern), ans)
	}

	{
		queries := []string{"FooBar", "FooBarTest", "FootBall", "FrameBuffer", "ForceFeedBack"}
		pattern := "FoBa"
		ans := []bool{true, false, true, false, false}
		assert(camelMatch(queries, pattern), ans)
	}

	{
		queries := []string{"FooBar", "FooBarTest", "FootBall", "FrameBuffer", "ForceFeedBack"}
		pattern := "FoBaT"
		ans := []bool{false, true, false, false, false}
		assert(camelMatch(queries, pattern), ans)
	}
}
