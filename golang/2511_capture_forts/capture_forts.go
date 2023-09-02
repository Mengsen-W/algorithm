/*
 * @Date: 2023-09-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-02
 * @FilePath: /algorithm/golang/2511_capture_forts/capture_forts.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func captureForts(forts []int) int {
	max := func(a int, b int) int {
		if a > b {
			return a
		}
		return b
	}
	ans, pre := 0, -1
	for i, fort := range forts {
		if fort == -1 || fort == 1 {
			if pre >= 0 && forts[pre] != fort {
				ans = max(ans, i-pre-1)
			}
			pre = i
		}
	}
	return ans
}

func main() {
	tests := []struct {
		forts []int
		ans   int
	}{
		{[]int{1, 0, 0, -1, 0, 0, 0, 0, 1}, 4},
		{[]int{0, 0, 1, -1}, 0},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, captureForts(item.forts))
	}
}
