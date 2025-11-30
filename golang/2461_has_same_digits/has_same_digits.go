// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func hasSameDigits(s string) bool {
	n := len(s)
	arr := []byte(s)
	for i := 1; i <= n-2; i++ {
		for j := 0; j <= n-1-i; j++ {
			arr[j] = byte(((int(arr[j]-'0') + int(arr[j+1]-'0')) % 10) + '0')
		}
	}
	return arr[0] == arr[1]
}

func main() {
	tests := []struct {
		s   string
		ans bool
	}{
		{"3902", true},
		{"34789", false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, hasSameDigits(test.s), index)
	}
}
