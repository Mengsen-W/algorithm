// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxFreqSum(s string) int {
	mp := make(map[byte]int)
	for i := 0; i < len(s); i++ {
		mp[s[i]]++
	}

	vowel, consonant := 0, 0
	for ch := 'a'; ch <= 'z'; ch++ {
		count := mp[byte(ch)]
		if isVowel(byte(ch)) {
			vowel = max(vowel, count)
		} else {
			consonant = max(consonant, count)
		}
	}
	return vowel + consonant
}

func isVowel(c byte) bool {
	return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"successes", 6},
		{"aeiaeia", 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxFreqSum(test.s), index)
	}
}
