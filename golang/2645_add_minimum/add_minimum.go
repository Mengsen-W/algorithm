/*
 * @Date: 2024-01-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-11
 * @FilePath: /algorithm/golang/2645_add_minimum/add_minimum.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func addMinimum(word string) int {
	n, cnt := len(word), 1
	for i := 1; i < n; i++ {
		if word[i] <= word[i-1] {
			cnt++
		}
	}
	return cnt*3 - n
}

func main() {
	tests := []struct {
		word string
		ans  int
	}{
		{"b", 2},
		{"aaa", 6},
		{"abc", 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, addMinimum(test.word), index)
	}
}
