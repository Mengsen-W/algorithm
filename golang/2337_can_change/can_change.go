/*
 * @Date: 2023-08-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-21
 * @FilePath: /algorithm/golang/2337_can_change/can_change.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func canChange(start string, target string) bool {
	i, j, n := 0, 0, len(start)
	for i < n && j < n {
		for i < n && start[i] == '_' {
			i++
		}
		for j < n && target[j] == '_' {
			j++
		}
		if i < n && j < n {
			if start[i] != target[j] {
				return false
			}
			c := start[i]
			if c == 'L' && i < j || c == 'R' && i > j {
				return false
			}
			i++
			j++
		}
	}
	for i < n {
		if start[i] != '_' {
			return false
		}
		i++
	}
	for j < n {
		if target[j] != '_' {
			return false
		}
		j++
	}
	return true
}

func main() {
	tests := []struct {
		start  string
		target string
		ans    bool
	}{
		{"_L__R__R_", "L______RR", true},
		{"R_L_", "__LR", false},
		{"_R", "R_", false},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, canChange(item.start, item.target))
	}
}
