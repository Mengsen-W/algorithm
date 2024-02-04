/*
 * @Date: 2021-09-18 08:50:02
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-04
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func canWinNim(n int) bool {
	return n%4 != 0
}

func main() {
	tests := []struct {
		n   int
		ans bool
	}{
		{4, false},
		{1, true},
		{2, true},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, canWinNim(test.n), index)
	}
}
