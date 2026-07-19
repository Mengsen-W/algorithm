// Package main ...
package main

import "fmt"

func smallestSubsequence(s string) string {
	left := [26]int{}
	for _, ch := range s {
		left[ch-'a']++
	}
	stack := []byte{}
	inStack := [26]bool{}
	for i := range s {
		ch := s[i]
		if !inStack[ch-'a'] {
			for len(stack) > 0 && ch < stack[len(stack)-1] {
				last := stack[len(stack)-1] - 'a'
				if left[last] == 0 {
					break
				}
				stack = stack[:len(stack)-1]
				inStack[last] = false
			}
			stack = append(stack, ch)
			inStack[ch-'a'] = true
		}
		left[ch-'a']--
	}
	return string(stack)
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"bcabc", "abc"},
		{"cbacdcbc", "acdb"},
	}

	for _, test := range tests {
		result := smallestSubsequence(test.s)
		if result != test.ans {
			fmt.Printf("smallestSubsequence(%s) = %s, want %s\n", test.s, result, test.ans)
		}
	}
}
