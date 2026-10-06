// Package main ...
package main

func minAddToMakeValid(s string) (ans int) {
	cnt := 0
	for _, c := range s {
		if c == '(' {
			cnt++
		} else if cnt > 0 {
			cnt--
		} else {
			ans++
		}
	}
	return ans + cnt
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
		{"())", 1},
		{"(((", 3},
	}

	for _, test := range tests {
		assert(minAddToMakeValid(test.s) == test.ans)
	}
}
