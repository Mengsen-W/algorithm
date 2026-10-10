// Package main ...
package main

func minInsertions(s string) int {
	insertions := 0
	leftCount := 0
	length := len(s)
	index := 0

	for index < length {
		c := s[index]
		if c == '(' {
			leftCount++
			index++
		} else {
			if leftCount > 0 {
				leftCount--
			} else {
				insertions++
			}

			if index < length-1 && s[index+1] == ')' {
				index += 2
			} else {
				insertions++
				index++
			}
		}
	}

	insertions += leftCount * 2
	return insertions
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"(()))", 1},
		{"())", 0},
		{"))())(", 3},
		{"((((((", 12},
		{")))))))", 5},
	}

	for _, test := range tests {
		ans := minInsertions(test.s)
		if ans != test.ans {
			panic("Wrong answer")
		}
	}
}
