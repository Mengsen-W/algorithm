// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumOperations(num string) int {
	n := len(num)
	find0, find5 := false, false
	for i := n - 1; i >= 0; i-- {
		if num[i] == '0' || num[i] == '5' {
			if find0 {
				return n - i - 2
			}
			if num[i] == '0' {
				find0 = true
			} else {
				find5 = true
			}
		} else if num[i] == '2' || num[i] == '7' {
			if find5 {
				return n - i - 2
			}
		}
	}
	if find0 {
		return n - 1
	}
	return n
}

func main() {
	tests := []struct {
		num string
		ans int
	}{
		{"2245047", 2},
		{"2908305", 3},
		{"10", 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumOperations(test.num), index)
	}
}
