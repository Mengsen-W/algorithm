// package main ...
package main

import (
	"math/bits"
	"testing"

	"github.com/stretchr/testify/assert"
)

func longestSubsequence(s string, k int) int {
	sm := 0
	cnt := 0
	bits := bits.Len(uint(k))
	for i := 0; i < len(s); i++ {
		ch := s[len(s)-1-i]
		if ch == '1' {
			if i < bits && sm+(1<<i) <= k {
				sm += 1 << i
				cnt++
			}
		} else {
			cnt++
		}
	}
	return cnt
}

func main() {
	tests := []struct {
		s   string
		k   int
		ans int
	}{
		{"1001010", 5, 5},
		{"00101001", 1, 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, longestSubsequence(test.s, test.k), index)
	}
}
