/*
 * @Date: 2023-07-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-17
 * @FilePath: /algorithm/golang/415_add_strings/add_strings.go
 */

// Package main ...
package main

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func addStrings(num1 string, num2 string) string {
	add := 0
	ans := ""
	for i, j := len(num1)-1, len(num2)-1; i >= 0 || j >= 0 || add != 0; i, j = i-1, j-1 {
		var x, y int
		if i >= 0 {
			x = int(num1[i] - '0')
		}
		if j >= 0 {
			y = int(num2[j] - '0')
		}
		result := x + y + add
		ans = strconv.Itoa(result%10) + ans
		add = result / 10
	}
	return ans
}

func main() {
	tests := []struct {
		num1 string
		num2 string
		ans  string
	}{
		{"11", "123", "134"},
		{"456", "77", "533"},
		{"0", "0", "0"},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, addStrings(item.num1, item.num2))
	}
}
