/*
 * @Date: 2022-09-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-26
 * @FilePath: /algorithm/golang/828_unique_letter_string/unique_letter_string.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func uniqueLetterString(s string) (ans int) {
	idx := map[rune][]int{}
	for i, c := range s {
		idx[c] = append(idx[c], i)
	}
	for _, arr := range idx {
		arr = append(append([]int{-1}, arr...), len(s))
		for i := 1; i < len(arr)-1; i++ {
			ans += (arr[i] - arr[i-1]) * (arr[i+1] - arr[i])
		}
	}
	return
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"ABC", 10},
		{"ABA", 8},
		{"LEETCODE", 92},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, uniqueLetterString(test.s), index)
	}
}
