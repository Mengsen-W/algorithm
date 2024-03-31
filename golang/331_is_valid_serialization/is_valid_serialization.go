/*
 * @Date: 2024-03-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-31
 * @FilePath: /algorithm/golang/331_is_valid_serialization/is_valid_serialization.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isValidSerialization(preorder string) bool {
	n := len(preorder)
	slots := 1
	for i := 0; i < n; {
		if slots == 0 {
			return false
		}
		if preorder[i] == ',' {
			i++
		} else if preorder[i] == '#' {
			slots--
			i++
		} else {
			// 读一个数字
			for i < n && preorder[i] != ',' {
				i++
			}
			slots++ // slots = slots - 1 + 2
		}
	}
	return slots == 0
}

func main() {
	assert.True(&testing.T{}, isValidSerialization("9,3,4,#,#,1,#,#,2,#,6,#,#"))
	assert.False(&testing.T{}, isValidSerialization("1,#"))
	assert.False(&testing.T{}, isValidSerialization("9, #, #, 1"))
	assert.True(&testing.T{}, isValidSerialization("9,#,92,#,#"))
}
