/*
 * @Date: 2021-05-26 09:53:17
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-26 10:01:00
 */

package main

func reverseParentheses(s string) string {
	n := len(s)
	pair := make([]int, n)
	stack := []int{}
	for i, b := range s {
		if b == '(' {
			stack = append(stack, i)
		} else if b == ')' {
			j := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			pair[i], pair[j] = j, i
		}
	}

	ans := []byte{}
	for i, step := 0, 1; i < n; i += step {
		if s[i] == '(' || s[i] == ')' {
			i = pair[i]
			step = -step
		} else {
			ans = append(ans, s[i])
		}
	}
	return string(ans)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(reverseParentheses("(abcd)") == "dcba")
	assert(reverseParentheses("(u(love)i)") == "iloveu")
	assert(reverseParentheses("(ed(et(oc))el)") == "leetcode")
	assert(reverseParentheses("a(bcdefghijkl(mno)p)q") == "apmnolkjihgfedcbq")
}
