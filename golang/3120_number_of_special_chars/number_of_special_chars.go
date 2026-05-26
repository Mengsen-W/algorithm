// Package main ...
package main

func numberOfSpecialChars(word string) int {
	s := make(map[rune]bool)
	for _, c := range word {
		s[c] = true
	}
	ans := 0
	for c := 'a'; c <= 'z'; c++ {
		if s[c] && s[c-'a'+'A'] {
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		word string
		ans  int
	}{
		{"aaAbcBC", 3},
		{"abc", 0},
		{"abBCab", 1},
	}

	for _, test := range tests {
		if got := numberOfSpecialChars(test.word); got != test.ans {
			panic(test)
		}
	}
}
