// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func checkPowersOfThree(n int) bool {
	for ; n > 0; n /= 3 {
		if n%3 == 2 {
			return false
		}
	}
	return true
}

func main() {
	tests := []struct {
		n      int
		except bool
	}{
		{12, true},
		{91, true},
		{21, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.except, checkPowersOfThree(test.n), index)
	}
}
