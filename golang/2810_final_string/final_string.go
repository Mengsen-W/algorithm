/*
 * @Date: 2024-04-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-01
 * @FilePath: /algorithm/golang/2810_final_string/final_string.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func finalString(s string) string {
	reverse := func(arr []rune) {
		for i, j := 0, len(arr)-1; i < j; i, j = i+1, j-1 {
			arr[i], arr[j] = arr[j], arr[i]
		}
	}
	q := []rune{}
	head := false
	for _, ch := range s {
		if ch != 'i' {
			if head {
				q = append([]rune{ch}, q...)
			} else {
				q = append(q, ch)
			}
		} else {
			head = !head
		}
	}
	if head {
		reverse(q)
	}
	return string(q)
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"string", "rtsng"},
		{"poiinter", "ponter"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, finalString(test.s), index)
	}
}
