// Package main ...
package main

func maxDepth(s string) (ans int) {
	size := 0
	for _, ch := range s {
		if ch == '(' {
			size++
			if size > ans {
				ans = size
			}
		} else if ch == ')' {
			size--
		}
	}
	return
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}
	tests := []struct {
		s   string
		ans int
	}{
		{"(1+(2*3)+((8)/4))+1", 3},
		{"(1)+((2))+(((3)))", 3},
		{"1+(2*3)/(2-1)", 1},
		{"1", 0},
	}

	for _, test := range tests {
		assert(maxDepth(test.s), test.ans)
	}
}
