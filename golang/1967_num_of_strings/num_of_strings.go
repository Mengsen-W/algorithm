// Package main ...
package main

import "fmt"

func numOfStrings(patterns []string, word string) int {
	check := func(pattern string, word string) bool {
		m := len(pattern)
		n := len(word)
		for i := 0; i+m <= n; i++ {
			flag := true
			for j := 0; j < m; j++ {
				if word[i+j] != pattern[j] {
					flag = false
					break
				}
			}
			if flag {
				return true
			}
		}
		return false
	}

	res := 0
	for _, pattern := range patterns {
		if check(pattern, word) {
			res++
		}
	}
	return res
}

func main() {
	tests := []struct {
		patterns []string
		word     string
		ans      int
	}{
		{[]string{"a", "abc", "bc", "d"}, "abc", 3},
		{[]string{"a", "b", "c"}, "aaaaabbbbb", 2},
		{[]string{"a", "a", "a"}, "ab", 3},
	}

	for _, test := range tests {
		res := numOfStrings(test.patterns, test.word)
		if res != test.ans {
			fmt.Println("test failed", test.patterns, test.word, res, test.ans)
		}
	}
}
