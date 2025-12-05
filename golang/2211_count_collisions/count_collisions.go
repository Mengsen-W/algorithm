// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countCollisions(directions string) int {
	res := 0
	flag := -1
	for _, c := range directions {
		if c == 'L' {
			if flag >= 0 {
				res += flag + 1
				flag = 0
			}
		} else if c == 'S' {
			if flag > 0 {
				res += flag
			}
			flag = 0
		} else {
			if flag >= 0 {
				flag++
			} else {
				flag = 1
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		directions string
		res        int
	}{
		{"RLRSLL", 5},
		{"LLRR", 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.res, countCollisions(test.directions), "case %d", index)
	}
}
