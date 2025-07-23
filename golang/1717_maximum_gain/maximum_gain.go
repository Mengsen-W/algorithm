// package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumGain(s string, x int, y int) (ans int) {
	a, b := 'a', 'b'
	if x < y {
		x, y = y, x
		a, b = b, a
	}

	var cnt1, cnt2 int
	for _, c := range s {
		if c == a {
			cnt1++
		} else if c == b {
			if cnt1 > 0 {
				ans += x
				cnt1--
			} else {
				cnt2++
			}
		} else {
			ans += min(cnt1, cnt2) * y
			cnt1, cnt2 = 0, 0
		}
	}
	ans += min(cnt1, cnt2) * y
	return
}

func main() {
	tests := []struct {
		s   string
		x   int
		y   int
		ans int
	}{
		{"cdbcbbaaabab", 4, 5, 19},
		{"aabbaaxybbaabb", 5, 4, 20},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumGain(test.s, test.x, test.y), "case %d", index)
	}
}
