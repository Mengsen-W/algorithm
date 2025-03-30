// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func longestCycle(edges []int) int {
	n := len(edges)
	label := make([]int, n)
	current_label, ans := 0, -1
	for i := 0; i < n; i++ {
		if label[i] != 0 {
			continue
		}
		pos, start_label := i, current_label
		for pos != -1 {
			current_label++
			// 如果遇到了已经遍历过的节点
			if label[pos] != 0 {
				// 如果该节点是这一次 i 循环中遍历的，说明找到了新的环，更新答案
				if label[pos] > start_label {
					if current_label-label[pos] > ans {
						ans = current_label - label[pos]
					}
				}
				break
			}
			label[pos] = current_label
			pos = edges[pos]
		}
	}
	return ans
}

func main() {
	tests := []struct {
		edges []int
		ans   int
	}{
		{[]int{3, 3, 4, 2, 3}, 3},
		{[]int{2, -1, 3, 1}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, longestCycle(test.edges), index)
	}
}
