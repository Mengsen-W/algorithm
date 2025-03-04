// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func checkPartitioning(s string) bool {
	n := len(s)
	isPalindrome := make([][]bool, n)
	for i := 0; i < n; i++ {
		isPalindrome[i] = make([]bool, n)
	}
	for length := 1; length < n; length++ {
		for start := 0; start <= n-length; start++ {
			end := start + length - 1
			if length == 1 {
				isPalindrome[start][end] = true
			} else if length == 2 {
				isPalindrome[start][end] = s[start] == s[end]
			} else {
				isPalindrome[start][end] = s[start] == s[end] && isPalindrome[start+1][end-1]
			}
		}
	}
	for start := 1; start < n-1; start++ {
		if !isPalindrome[0][start-1] {
			continue
		}
		for end := start; end < n-1; end++ {
			if isPalindrome[start][end] && isPalindrome[end+1][n-1] {
				return true
			}
		}
	}
	return false
}

func main() {
	tests := []struct {
		s   string
		ans bool
	}{
		{"abcbdd", true},
		{"bcbddxy", false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, checkPartitioning(test.s), index)
	}
}
