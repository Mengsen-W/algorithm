// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func finalValueAfterOperations(operations []string) (x int) {
	for _, op := range operations {
		if op[1] == '+' {
			x++
		} else {
			x--
		}
	}
	return
}

func main() {
	tests := []struct {
		operations []string
		ans        int
	}{
		{[]string{"--X", "X++", "X++"}, 1},
		{[]string{"++X", "++X", "X++"}, 3},
		{[]string{"X++", "++X", "--X", "X--"}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, finalValueAfterOperations(test.operations), "case %d", index)
	}
}
