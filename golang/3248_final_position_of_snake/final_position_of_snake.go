// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func finalPositionOfSnake(n int, commands []string) int {
	ans := 0
	for _, c := range commands {
		switch c[0] {
		case 'U':
			ans -= n
		case 'D':
			ans += n
		case 'L':
			ans--
		case 'R':
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		n        int
		commands []string
		ans      int
	}{
		{2, []string{"RIGHT", "DOWN"}, 3},
		{3, []string{"DOWN", "RIGHT", "UP"}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, finalPositionOfSnake(test.n, test.commands), index)
	}
}
