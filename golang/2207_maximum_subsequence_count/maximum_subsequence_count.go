// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumSubsequenceCount(text string, pattern string) int64 {
	var res, cnt1, cnt2 int64
	for _, c := range text {
		if byte(c) == pattern[1] {
			res += cnt1
			cnt2++
		}
		if byte(c) == pattern[0] {
			cnt1++
		}
	}
	if cnt1 > cnt2 {
		return res + cnt1
	}
	return res + cnt2
}

func main() {
	tests := []struct {
		text    string
		pattern string
		ans     int64
	}{
		{"abdcdbc", "ac", 4},
		{"aabb", "ab", 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumSubsequenceCount(test.text, test.pattern), index)
	}
}
