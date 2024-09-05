// Package main ...
package main

import (
	"testing"
	"unicode"

	"github.com/stretchr/testify/assert"
)

func clearDigits(s string) string {
	var res []byte
	for _, c := range s {
		if unicode.IsDigit(c) {
			res = res[:len(res)-1]
		} else {
			res = append(res, byte(c))
		}
	}
	return string(res)
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"abc", "abc"},
		{"cb34", ""},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, clearDigits(test.s), index)
	}
}
