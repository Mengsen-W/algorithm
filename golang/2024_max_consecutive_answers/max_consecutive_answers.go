/*
 * @Date: 2022-03-29 01:59:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-29 02:16:10
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxConsecutiveAnswers(answerKey string, k int) int {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	maxConsecutiveChar := func(answerKey string, k int, ch byte) (ans int) {
		left, sum := 0, 0
		for right := range answerKey {
			if answerKey[right] != ch {
				sum++
			}
			for sum > k {
				if answerKey[left] != ch {
					sum--
				}
				left++
			}
			ans = max(ans, right-left+1)
		}
		return
	}
	return max(maxConsecutiveChar(answerKey, k, 'T'),
		maxConsecutiveChar(answerKey, k, 'F'))
}

func main() {
	tests := []struct {
		answerKey string
		k         int
		ans       int
	}{
		{"TTFF", 2, 4},
		{"TFFT", 1, 3},
		{"TTFTTFTT", 1, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxConsecutiveAnswers(test.answerKey, test.k), index)
	}
}
