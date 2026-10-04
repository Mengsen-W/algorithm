// Package main ...
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

	tests := []struct {
		s   string
		ans int
	}{
		{"()", 1},
		{"(())", 2},
		{"()()", 2},
		{"(()(()))", 6},
	}

	for _, test := range tests {
		assert(scoreOfParentheses(test.s) == test.ans)
	}
}
