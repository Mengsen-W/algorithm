/*
 * @Date: 2024-01-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-10
 * @FilePath: /algorithm/golang/2696_min_length/min_length.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minLength(s string) int {
	var stack []byte
	for i := range s {
		stack = append(stack, s[i])
		m := len(stack)
		if m >= 2 && (string(stack[m-2:]) == "AB" || string(stack[m-2:]) == "CD") {
			stack = stack[:m-2]
		}
	}
	return len(stack)
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"ABFCACDB", 2},
		{"ACBBD", 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minLength(test.s), index)
	}
}
