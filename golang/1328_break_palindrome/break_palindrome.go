// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func breakPalindrome(palindrome string) string {
	n := len(palindrome)
	if n == 1 {
		return ""
	}
	data := []byte(palindrome)
	for i := 0; i*2+1 < n; i++ {
		if data[i] != 'a' {
			data[i] = 'a'
			return string(data)
		}
	}
	data[n-1]++
	return string(data)
}

func main() {
	tests := []struct {
		palindrome string
		ans        string
	}{
		{"abccba", "aaccba"},
		{"a", ""},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, breakPalindrome(test.palindrome), index)
	}
}
