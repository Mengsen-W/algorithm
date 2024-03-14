/*
 * @Date: 2024-03-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-13
 * @FilePath: /algorithm/golang/2864_maximum_odd_binary_number/maximum_odd_binary_number.go
 */

// Package main ...
package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumOddBinaryNumber(s string) string {
	cnt := strings.Count(s, "1")
	return strings.Repeat("1", cnt-1) + strings.Repeat("0", len(s)-cnt) + "1"
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"010", "001"},
		{"0101", "1001"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumOddBinaryNumber(test.s), index)
	}
}
