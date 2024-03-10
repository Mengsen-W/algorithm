/*
 * @Date: 2021-11-08 00:08:31
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-10
 */

package main

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func getHint(secret, guess string) string {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	bulls := 0
	var cntS, cntG [10]int
	for i := range secret {
		if secret[i] == guess[i] {
			bulls++
		} else {
			cntS[secret[i]-'0']++
			cntG[guess[i]-'0']++
		}
	}
	cows := 0
	for i := 0; i < 10; i++ {
		cows += min(cntS[i], cntG[i])
	}
	return fmt.Sprintf("%dA%dB", bulls, cows)
}

func main() {
	tests := []struct {
		secret string
		guess  string
		ans    string
	}{
		{"1807", "7810", "1A3B"},
		{"1123", "0111", "1A1B"},
		{"1", "0", "0A0B"},
		{"1", "1", "1A0B"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getHint(test.secret, test.guess), index)
	}
}
