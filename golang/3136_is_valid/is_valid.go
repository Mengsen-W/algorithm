// Package main ...
package main

import (
	"testing"
	"unicode"

	"github.com/stretchr/testify/assert"
)

func isValid(word string) bool {
	if len(word) < 3 {
		return false
	}
	hasVowel := false
	hasConsonant := false
	for _, c := range word {
		if unicode.IsLetter(c) {
			ch := unicode.ToLower(c)
			if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
				hasVowel = true
			} else {
				hasConsonant = true
			}
		} else if !unicode.IsDigit(c) {
			return false
		}
	}
	return hasVowel && hasConsonant
}

func main() {
	tests := []struct {
		word string
		ans  bool
	}{
		{"234Adas", true},
		{"b3", false},
		{"a3$e", false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isValid(test.word), index)
	}
}
