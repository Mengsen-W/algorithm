/*
 * @Date: 2023-01-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-12
 * @FilePath: /algorithm/1807_evaluate/evaluate.go
 */

package main

import "strings"

func evaluate(s string, knowledge [][]string) string {
	dict := map[string]string{}
	for _, kd := range knowledge {
		dict[kd[0]] = kd[1]
	}
	ans := &strings.Builder{}
	start := -1
	for i, c := range s {
		if c == '(' {
			start = i
		} else if c == ')' {
			if t, ok := dict[s[start+1:i]]; ok {
				ans.WriteString(t)
			} else {
				ans.WriteByte('?')
			}
			start = -1
		} else if start < 0 {
			ans.WriteRune(c)
		}
	}
	return ans.String()
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "(name)is(age)yearsold"
		knowledge := [][]string{{"name", "bob"}, {"age", "two"}}
		ans := "bobistwoyearsold"
		assert(evaluate(s, knowledge) == ans)
	}

	{
		s := "hi(name)"
		knowledge := [][]string{{"a", "b"}}
		ans := "hi?"
		assert(evaluate(s, knowledge) == ans)
	}

	{
		s := "(a)(a)(a)aaa"
		knowledge := [][]string{{"a", "yes"}}
		ans := "yesyesyesaaa"
		assert(evaluate(s, knowledge) == ans)
	}
}
