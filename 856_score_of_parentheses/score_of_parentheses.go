/*
 * @Date: 2022-10-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-09
 * @FilePath: /algorithm/856_score_of_parentheses/score_of_parentheses.go
 */

package main

func scoreOfParentheses(s string) (ans int) {
	bal := 0
	for i, c := range s {
		if c == '(' {
			bal++
		} else {
			bal--
			if s[i-1] == '(' {
				ans += 1 << bal
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

	assert(scoreOfParentheses("()") == 1)
	assert(scoreOfParentheses("(())") == 2)
	assert(scoreOfParentheses("()()") == 2)
	assert(scoreOfParentheses("(()(()))") == 6)
}
