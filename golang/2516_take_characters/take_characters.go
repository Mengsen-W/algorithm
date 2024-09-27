// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func takeCharacters(s string, k int) int {
	cnt := make([]int, 3)
	ans := len(s)

	for i := 0; i < len(s); i++ {
		cnt[s[i]-'a']++
	}

	if cnt[0] >= k && cnt[1] >= k && cnt[2] >= k {
		ans = min(ans, len(s))
	} else {
		return -1
	}

	l := 0
	for r := 0; r < len(s); r++ {
		cnt[s[r]-'a']--
		for l < r && (cnt[0] < k || cnt[1] < k || cnt[2] < k) {
			cnt[s[l]-'a']++
			l++
		}
		if cnt[0] >= k && cnt[1] >= k && cnt[2] >= k {
			ans = min(ans, len(s)-(r-l+1))
		}
	}

	return ans
}

func main() {
	tests := []struct {
		s   string
		k   int
		ans int
	}{
		{"aabaaaacaabc", 2, 8},
		{"a", 1, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, takeCharacters(test.s, test.k), index)
	}
}
