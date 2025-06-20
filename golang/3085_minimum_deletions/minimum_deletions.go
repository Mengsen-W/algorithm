// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumDeletions(word string, k int) int {
	cnt := make(map[rune]int)
	for _, ch := range word {
		cnt[ch]++
	}
	res := len(word)
	for _, a := range cnt {
		deleted := 0
		for _, b := range cnt {
			if a > b {
				deleted += b
			} else if b > a+k {
				deleted += b - (a + k)
			}
		}
		if deleted < res {
			res = deleted
		}
	}
	return res
}

func main() {
	tests := []struct {
		word string
		k    int
		ans  int
	}{
		{"aabcaba", 0, 3},
		{"dabdcbdcdcd", 2, 2},
		{"aaabaaa", 2, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumDeletions(test.word, test.k), index)
	}
}
