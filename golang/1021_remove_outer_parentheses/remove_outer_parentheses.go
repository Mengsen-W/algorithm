/*
 * @Date: 2022-05-28 10:30:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-28 10:38:35
 * @FilePath: /algorithm/1021_remove_outer_parentheses/remove_outer_parentheses.go
 */

package main

func removeOuterParentheses(s string) string {
	ans := []rune{}
	level := 0
	for _, c := range s {
		if c == ')' {
			level--
		}
		if level > 0 {
			ans = append(ans, c)
		}
		if c == '(' {
			level++
		}
	}
	return string(ans)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(removeOuterParentheses("(()())(())") == "()()()")
	assert(removeOuterParentheses("(()())(())(()(()))") == "()()()()(())")
	assert(removeOuterParentheses("()()") == "")
}
