/*
 * @Date: 2024-04-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-10
 * @FilePath: /algorithm/golang/1702_maximum_binary_string/maximum_binary_string.go
 */

// Package main ...
package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumBinaryString(binary string) string {
	n := len(binary)
	i := strings.Index(binary, "0")
	if i < 0 {
		return binary
	}
	zeros := strings.Count(binary, "0")
	s := []rune(strings.Repeat("1", n))
	s[i+zeros-1] = '0'
	return string(s)
}

func main() {
	tests := []struct {
		binary string
		ans    string
	}{
		{"000110", "111011"},
		{"01", "01"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumBinaryString(test.binary), index)
	}
}
