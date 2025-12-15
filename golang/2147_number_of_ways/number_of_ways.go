// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfWays(corridor string) int {
	const mod = 1e9 + 7
	prev, cnt, ans := -1, 0, 1
	for i, ch := range corridor {
		if ch == 'S' {
			cnt += 1
			if (cnt >= 3) && (cnt%2 == 1) {
				ans = ans * (i - prev) % mod
			}
			prev = i
		}
	}
	if (cnt < 2) || (cnt%2 == 1) {
		ans = 0
	}
	return ans
}

func main() {
	tests := []struct {
		corridor string
		ans      int
	}{
		{"SSPPSPS", 3},
		{"PPSPSP", 1},
		{"S", 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfWays(test.corridor), index)
	}
}
