// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func getSmallestString(s string, k int) string {
	runes := []rune(s)
	for i := 0; i < len(runes); i++ {
		dis := min(int(runes[i]-'a'), int('z'-runes[i]+1))
		if dis <= k {
			runes[i] = 'a'
			k -= dis
		} else {
			runes[i] = rune(int(runes[i]) - k)
			break
		}
	}
	return string(runes)
}

func main() {
	tests := []struct {
		s   string
		k   int
		ans string
	}{
		{"zbbz", 3, "aaaz"},
		{"xaxcd", 4, "aawcd"},
		{"lol", 0, "lol"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getSmallestString(test.s, test.k), index)
	}
}
