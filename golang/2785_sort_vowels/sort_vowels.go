// Package main ...
package main

import (
	"slices"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

var vowels = map[rune]bool{
	'a': true, 'e': true, 'i': true, 'o': true, 'u': true,
	'A': true, 'E': true, 'I': true, 'O': true, 'U': true,
}

func sortVowels(s string) string {
	var tmp []rune
	for _, ch := range s {
		if vowels[ch] {
			tmp = append(tmp, ch)
		}
	}

	slices.SortFunc(tmp, func(a, b rune) int {
		if a < b {
			return -1
		}
		if a > b {
			return 1
		}
		return 0
	})

	var result strings.Builder
	idx := 0
	for _, ch := range s {
		if vowels[ch] {
			result.WriteRune(tmp[idx])
			idx++
		} else {
			result.WriteRune(ch)
		}
	}

	return result.String()
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"lEetcOde", "lEOtcede"},
		{"lYmpH", "lYmpH"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, sortVowels(test.s), index)
	}
}
