// Package main ...
package main

import (
	"strconv"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minMaxDifference(num int) int {
	s := strconv.Itoa(num)
	t := s
	pos := 0
	for pos < len(s) && s[pos] == '9' {
		pos++
	}
	if pos < len(s) {
		s = strings.ReplaceAll(s, string(s[pos]), "9")
	}
	t = strings.ReplaceAll(t, string(t[0]), "0")
	max, _ := strconv.Atoi(s)
	min, _ := strconv.Atoi(t)
	return max - min
}

func main() {
	tests := []struct {
		num int
		ans int
	}{
		{11891, 99009},
		{90, 99},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minMaxDifference(test.num), index)
	}
}
