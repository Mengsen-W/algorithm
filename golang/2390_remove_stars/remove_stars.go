// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func removeStars(s string) string {
	var res []rune
	for _, c := range s {
		if c != '*' {
			res = append(res, c)
		} else {
			res = res[:len(res)-1]
		}
	}
	return string(res)
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"leet**cod*e", "lecoe"},
		{"erase*****", ""},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, removeStars(test.s), index)
	}
}
