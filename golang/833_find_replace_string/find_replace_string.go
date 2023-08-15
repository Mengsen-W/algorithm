/*
 * @Date: 2023-08-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-15
 * @FilePath: /algorithm/golang/833_find_replace_string/find_replace_string.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findReplaceString(s string, indices []int, sources []string, targets []string) string {
	min := func(x int, y int) int {
		if x > y {
			return y
		}
		return x
	}
	n, m := len(s), len(indices)

	ops := map[int][]int{}
	for i := 0; i < m; i++ {
		ops[indices[i]] = append(ops[i], i)
	}

	ans := ""
	for i := 0; i < n; {
		succeed := false
		_, ok := ops[i]
		if ok {
			for _, pt := range ops[i] {
				if s[i:i+min(len(sources[pt]), n-i)] == sources[pt] {
					succeed = true
					ans += targets[pt]
					i += len(sources[pt])
					break
				}
			}
		}
		if !succeed {
			ans += string(s[i])
			i++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		s       string
		indices []int
		source  []string
		target  []string
		ans     string
	}{
		{"abcd", []int{0, 2}, []string{"a", "cd"}, []string{"eee", "ffff"}, "eeebffff"},
		{"abcd", []int{0, 2}, []string{"ab", "ec"}, []string{"eee", "ffff"}, "eeecd"},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, findReplaceString(item.s, item.indices, item.source, item.target))
	}
}
