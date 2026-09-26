// Package main ...
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
	tests := []struct {
		s         string
		knowledge [][]string
		ans       string
	}{
		{"(name)is(age)yearsold", [][]string{{"name", "bob"}, {"age", "two"}}, "bobistwoyearsold"},
		{"hi(name)", [][]string{{"a", "b"}}, "hi?"},
		{"(a)(a)(a)aaa", [][]string{{"a", "yes"}}, "yesyesyesaaa"},
	}

	for _, test := range tests {
		if got := evaluate(test.s, test.knowledge); got != test.ans {
			panic("Not Passed")
		}
	}
}
