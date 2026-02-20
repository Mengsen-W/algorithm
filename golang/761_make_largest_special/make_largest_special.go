// Package main ...
package main

import (
	"sort"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func makeLargestSpecial(s string) string {
	if len(s) <= 2 {
		return s
	}
	subs := sort.StringSlice{}
	cnt, left := 0, 0
	for i, ch := range s {
		if ch == '1' {
			cnt++
		} else if cnt--; cnt == 0 {
			subs = append(subs, "1"+makeLargestSpecial(s[left+1:i])+"0")
			left = i + 1
		}
	}
	sort.Sort(sort.Reverse(subs))
	return strings.Join(subs, "")
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"11011000", "11100100"},
		{"10", "10"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, makeLargestSpecial(test.s), index)
	}
}
