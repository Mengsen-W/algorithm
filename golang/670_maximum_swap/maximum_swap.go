/*
 * @Date: 2022-09-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-22
 * @FilePath: /algorithm/golang/670_maximum_swap/maximum_swap.go
 */

package main

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumSwap(num int) int {
	s := []byte(strconv.Itoa(num))
	n := len(s)
	maxIdx, idx1, idx2 := n-1, -1, -1
	for i := n - 1; i >= 0; i-- {
		if s[i] > s[maxIdx] {
			maxIdx = i
		} else if s[i] < s[maxIdx] {
			idx1, idx2 = i, maxIdx
		}
	}
	if idx1 < 0 {
		return num
	}
	s[idx1], s[idx2] = s[idx2], s[idx1]
	v, _ := strconv.Atoi(string(s))
	return v
}

func main() {
	tests := []struct {
		num int
		ans int
	}{
		{2736, 7236},
		{9973, 9973},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumSwap(test.num), index)
	}
}
