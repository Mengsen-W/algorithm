/*
 * @Date: 2023-06-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-30
 * @FilePath: /algorithm/golang/2490_is_circular_sentence/is_circular_sentence.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isCircularSentence(sentence string) bool {
	n := len(sentence)
	if sentence[n-1] != sentence[0] {
		return false
	}
	for i := 0; i < n; i++ {
		if sentence[i] == ' ' && sentence[i+1] != sentence[i-1] {
			return false
		}
	}
	return true
}

func main() {
	testMap := []struct {
		sentence string
		ans      bool
	}{
		{"leetcode exercises sound delightful", true},
		{"eetcode", true},
		{"Leetcode is cool", false},
	}

	for _, item := range testMap {
		assert.Equal(&testing.T{}, isCircularSentence(item.sentence), item.ans)
	}
}
