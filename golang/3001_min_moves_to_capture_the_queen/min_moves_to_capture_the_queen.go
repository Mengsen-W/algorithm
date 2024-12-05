// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minMovesToCaptureTheQueen(a int, b int, c int, d int, e int, f int) int {
	// 车与皇后处在同一行，且中间没有象
	if a == e && (c != a || d <= min(b, f) || d >= max(b, f)) {
		return 1
	}
	// 车与皇后处在同一列，且中间没有象
	if b == f && (d != b || c <= min(a, e) || c >= max(a, e)) {
		return 1
	}
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	// 象、皇后处在同一条对角线，且中间没有车
	if abs(c-e) == abs(d-f) && ((c-e)*(b-f) != (a-e)*(d-f) || a < min(c, e) || a > max(c, e)) {
		return 1
	}
	return 2
}

func main() {
	tests := []struct {
		a   int
		b   int
		c   int
		d   int
		e   int
		f   int
		ans int
	}{
		{1, 1, 8, 8, 2, 3, 2},
		{5, 3, 3, 4, 5, 2, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minMovesToCaptureTheQueen(test.a, test.b, test.c, test.d, test.e, test.f), index)
	}
}
