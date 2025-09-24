// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minSwaps(s string) int {
	cnt, mincnt := 0, 0
	for _, ch := range s {
		if ch == '[' {
			cnt++
		} else {
			cnt--
			mincnt = min(mincnt, cnt)
		}
	}
	return (-mincnt + 1) / 2
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"][][", 1},
		{"]]][[[", 2},
		{"[]", 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minSwaps(test.s), index)
	}
}
