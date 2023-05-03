/*
 * @Date: 2023-05-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-03
 * @FilePath: /algorithm/golang/1003_is_valid/is_valid.go
 */

// Package main ...
package main

func isValid(s string) bool {
	stk := []byte{}
	for i := range s {
		stk = append(stk, s[i])
		if len(stk) >= 3 && string(stk[len(stk)-3:]) == "abc" {
			stk = stk[:len(stk)-3]
		}
	}
	return len(stk) == 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "aabcbc"
		ans := true
		assert(isValid(s) == ans)
	}

	{
		s := "abcabcababcc"
		ans := true
		assert(isValid(s) == ans)
	}

	{
		s := "abccba"
		ans := false
		assert(isValid(s) == ans)
	}
}
