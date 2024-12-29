// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minAnagramLength(s string) int {
	n := len(s)
	check := func(m int) bool {
		var count0 [26]int
		for j := 0; j < n; j += m {
			var count1 [26]int
			for k := j; k < j+m; k++ {
				count1[s[k]-'a']++
			}
			if j > 0 && count0 != count1 {
				return false
			}
			count0 = count1
		}
		return true
	}
	for i := 1; i < n; i++ {
		if n%i != 0 {
			continue
		}
		if check(i) {
			return i
		}
	}
	return n
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"abba", 2},
		{"cdef", 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minAnagramLength(test.s), index)
	}
}