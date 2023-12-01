/*
 * @Date: 2023-12-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-01
 * @FilePath: /algorithm/golang/1657_close_strings/close_strings.go
 */

// Package main ...
package main

import (
	"reflect"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func closeStrings(word1 string, word2 string) bool {
	count1, count2 := make([]int, 26), make([]int, 26)
	for _, c := range word1 {
		count1[c-'a']++
	}
	for _, c := range word2 {
		count2[c-'a']++
	}
	for i := 0; i < 26; i++ {
		if count1[i] > 0 && count2[i] == 0 || count1[i] == 0 && count2[i] > 0 {
			return false
		}
	}
	sort.Ints(count1)
	sort.Ints(count2)
	return reflect.DeepEqual(count1, count2)
}

func main() {
	tests := []struct {
		word1 string
		word2 string
		ans   bool
	}{
		{"abc", "bca", true},
		{"a", "aa", false},
		{"cabbba", "abbccc", true},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, closeStrings(test.word1, test.word2), index)
	}
}
