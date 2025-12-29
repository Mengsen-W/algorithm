// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func pyramidTransition(bottom string, allowed []string) bool {
	// 构建转换表，T[i][j] 表示底部为i和j时，顶部可能的字符位掩码
	T := [7][7]int{}
	for _, a := range allowed {
		left := int(a[0] - 'A')
		right := int(a[1] - 'A')
		top := int(a[2] - 'A')
		T[left][right] |= 1 << top
	}

	seen := make(map[uint64]bool)
	n := len(bottom)
	// 金字塔状态数组
	A := make([][]int, n)
	for i := range A {
		A[i] = make([]int, n)
	}
	// 初始化底部行
	for i := 0; i < n; i++ {
		A[n-1][i] = int(bottom[i] - 'A')
	}

	var solve func(uint64, int, int) bool
	solve = func(R uint64, N int, i int) bool {
		// 基本情况：成功构建到金字塔顶部
		if N == 1 && i == 1 {
			return true
		} else if i == N { // 当前行处理完成，准备处理下一行
			// 记忆化检查：如果已经处理过相同的行状态，直接返回失败
			if seen[R] {
				return false
			}
			// 记录当前行状态
			seen[R] = true
			// 递归处理下一行
			return solve(0, N-1, 0)
		} else { // 处理当前行的当前位置
			// 获取当前两个底部块对应的可能顶部块位掩码
			w := T[A[N][i]][A[N][i+1]]
			// 遍历所有可能的顶部块
			for b := 0; b < 7; b++ {
				if (w>>b)&1 != 0 {
					// 设置顶部块
					A[N-1][i] = b
					// 递归处理下一个位置，更新状态编码
					// 使用base-8编码来记录当前行的状态
					if solve(R*8+uint64(b+1), N, i+1) {
						return true
					}
				}
			}
			return false
		}
	}

	return solve(0, n-1, 0)
}

func main() {
	tests := []struct {
		bottom  string
		allowed []string
		ans     bool
	}{
		{"BCD", []string{"BCC", "CDE", "CEA", "FFF"}, true},
		{"AAAA", []string{"AAB", "AAC", "BCD", "BBE", "DEF"}, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, pyramidTransition(test.bottom, test.allowed), index)
	}
}
