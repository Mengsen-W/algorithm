/*
 * @Date: 2022-03-05 00:33:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-05 00:48:17
 * @FilePath: /algorithm/521_find_lu_slength/find_lu_slength.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findLUSlength(a, b string) int {
	if a != b {
		return func(a, b int) int {
			if b > a {
				return b
			}
			return a
		}(len(a), len(b))
	}
	return -1
}

func main() {
	tests := []struct {
		a   string
		b   string
		ans int
	}{
		{"aba", "cdc", 3},
		{"aaa", "bbb", 3},
		{"aaa", "aaa", -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findLUSlength(test.a, test.b), index)
	}
}
