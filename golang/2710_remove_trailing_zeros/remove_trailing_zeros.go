// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func removeTrailingZeros(num string) string {
	n := len(num)
	for n > 0 && num[n-1] == '0' {
		n--
	}
	return num[0:n]
}

func main() {
	tests := []struct {
		num string
		ans string
	}{
		{"51230100", "512301"},
		{"123", "123"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, removeTrailingZeros(test.num), index)
	}
}
