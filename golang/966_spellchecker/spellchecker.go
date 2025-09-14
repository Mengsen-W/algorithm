// Package main ...
package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func spellchecker(wordlist []string, queries []string) []string {
	wordsPerfect := make(map[string]bool)
	wordsCap := make(map[string]string)
	wordsVow := make(map[string]string)

	for _, word := range wordlist {
		wordsPerfect[word] = true
		wordLow := strings.ToLower(word)
		if _, exists := wordsCap[wordLow]; !exists {
			wordsCap[wordLow] = word
		}
		wordLowDV := devowel(wordLow)
		if _, exists := wordsVow[wordLowDV]; !exists {
			wordsVow[wordLowDV] = word
		}
	}

	result := make([]string, len(queries))
	for i, query := range queries {
		result[i] = solve(query, wordsPerfect, wordsCap, wordsVow)
	}
	return result
}

func solve(query string, wordsPerfect map[string]bool, wordsCap, wordsVow map[string]string) string {
	if wordsPerfect[query] {
		return query
	}
	queryLow := strings.ToLower(query)
	if word, exists := wordsCap[queryLow]; exists {
		return word
	}
	queryLowDV := devowel(queryLow)
	if word, exists := wordsVow[queryLowDV]; exists {
		return word
	}

	return ""
}

func devowel(word string) string {
	var sb strings.Builder
	for _, c := range word {
		if isVowel(c) {
			sb.WriteRune('*')
		} else {
			sb.WriteRune(c)
		}
	}
	return sb.String()
}

func isVowel(c rune) bool {
	switch c {
	case 'a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U':
		return true
	}
	return false
}

func main() {
	tests := []struct {
		wordlist []string
		queries  []string
		ans      []string
	}{
		{
			[]string{"KiTe", "kite", "hare", "Hare"},
			[]string{"kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto"},
			[]string{"kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe"},
		},
		{
			[]string{"yellow"},
			[]string{"YellOw"},
			[]string{"yellow"},
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, spellchecker(test.wordlist, test.queries), index)
	}
}
