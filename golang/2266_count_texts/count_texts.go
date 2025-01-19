// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countTexts(pressedKeys string) int {
	m := 1000000007
	n := len(pressedKeys)
	dp3 := []int{1, 1, 2, 4} // 连续按多次 3 个字母按键对应的方案数
	dp4 := []int{1, 1, 2, 4} // 连续按多次 4 个字母按键对应的方案数
	for i := 4; i <= n; i++ {
		dp3 = append(dp3, (dp3[i-1]+dp3[i-2]+dp3[i-3])%m)
		dp4 = append(dp4, (dp4[i-1]+dp4[i-2]+dp4[i-3]+dp4[i-4])%m)
	}
	res := 1 // 总方案数
	cnt := 1 // 当前字符连续出现的次数
	for i := 1; i < n; i++ {
		if pressedKeys[i] == pressedKeys[i-1] {
			cnt++
		} else {
			// 对按键对应字符数量讨论并更新总方案数
			if pressedKeys[i-1] == '7' || pressedKeys[i-1] == '9' {
				res = (res * dp4[cnt]) % m
			} else {
				res = (res * dp3[cnt]) % m
			}
			cnt = 1
		}
	}
	// 更新最后一段连续字符子串对应的方案数
	if pressedKeys[n-1] == '7' || pressedKeys[n-1] == '9' {
		res = (res * dp4[cnt]) % m
	} else {
		res = (res * dp3[cnt]) % m
	}
	return res
}

func main() {
	tests := []struct {
		pressedKeys string
		ans         int
	}{
		{"22233", 8},
		{"222222222222222222222222222222222222", 82876089},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countTexts(test.pressedKeys), index)
	}
}
