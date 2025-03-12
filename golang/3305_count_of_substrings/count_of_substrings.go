// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countOfSubstrings(word string, k int) int64 {
	vowels := map[byte]bool{'a': true, 'e': true, 'i': true, 'o': true, 'u': true}
	count := func(m int) int64 {
		n := len(word)
		var res int64 = 0
		consonants := 0
		occur := make(map[byte]int)
		for i, j := 0, 0; i < n; i++ {
			for j < n && (consonants < m || len(occur) < 5) {
				if vowels[word[j]] {
					occur[word[j]]++
				} else {
					consonants++
				}
				j++
			}
			if consonants >= m && len(occur) == 5 {
				res += int64(n - j + 1)
			}
			if vowels[word[i]] {
				occur[word[i]]--
				if occur[word[i]] == 0 {
					delete(occur, word[i])
				}
			} else {
				consonants--
			}
		}
		return res
	}
	return count(k) - count(k+1)
}

func main() {
	tests := []struct {
		word string
		k    int
		ans  int64
	}{
		{"aeioqq", 1, 0},
		{"aeiou", 0, 1},
		{"ieaouqqieaouqq", 1, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countOfSubstrings(test.word, test.k), index)
	}
}
