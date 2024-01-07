/*
 * @Date: 2021-12-04 05:41:55
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-07
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func canConstruct(ransomNote, magazine string) bool {
	if len(ransomNote) > len(magazine) {
		return false
	}
	cnt := [26]int{}
	for _, ch := range magazine {
		cnt[ch-'a']++
	}
	for _, ch := range ransomNote {
		cnt[ch-'a']--
		if cnt[ch-'a'] < 0 {
			return false
		}
	}
	return true
}

func main() {
	tests := []struct {
		ransomNote string
		magazine   string
		ans        bool
	}{
		{"a", "b", false},
		{"aa", "ab", false},
		{"aa", "aab", true},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, canConstruct(test.ransomNote, test.magazine), index)
	}
}
