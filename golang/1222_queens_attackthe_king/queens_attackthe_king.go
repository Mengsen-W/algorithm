/*
 * @Date: 2023-09-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-15
 * @FilePath: /algorithm/golang/1222_queens_attackthe_king/queens_attackthe_king.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func queensAttacktheKing(queens [][]int, king []int) [][]int {
	queenPos := make(map[int]bool)
	for _, queen := range queens {
		x, y := queen[0], queen[1]
		queenPos[x*8+y] = true
	}

	ans := [][]int{}
	for dx := -1; dx <= 1; dx++ {
		for dy := -1; dy <= 1; dy++ {
			if dx == 0 && dy == 0 {
				continue
			}
			kx, ky := king[0]+dx, king[1]+dy
			for kx >= 0 && kx < 8 && ky >= 0 && ky < 8 {
				pos := kx*8 + ky
				if _, ok := queenPos[pos]; ok {
					ans = append(ans, []int{kx, ky})
					break
				}
				kx += dx
				ky += dy
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		queens [][]int
		king   []int
		ans    [][]int
	}{
		{[][]int{{0, 1}, {1, 0}, {4, 0}, {0, 4}, {3, 3}, {2, 4}}, []int{0, 0}, [][]int{{0, 1}, {1, 0}, {3, 3}}},
		{[][]int{{0, 0}, {1, 1}, {2, 2}, {3, 4}, {3, 5}, {4, 4}, {4, 5}}, []int{3, 3}, [][]int{{2, 2}, {3, 4}, {4, 4}}},
		{[][]int{{5, 6}, {7, 7}, {2, 1}, {0, 7}, {1, 6}, {5, 1}, {3, 7}, {0, 3}, {4, 0}, {1, 2}, {6, 3}, {5, 0}, {0, 4}, {2, 2}, {1, 1}, {6, 4}, {5, 4}, {0, 0}, {2, 6}, {4, 5}, {5, 2}, {1, 4}, {7, 5}, {2, 3}, {0, 5}, {4, 2}, {1, 0}, {2, 7}, {0, 1}, {4, 6}, {6, 1}, {0, 6}, {4, 3}, {1, 7}}, []int{3, 4}, [][]int{{2, 3}, {1, 4}, {1, 6}, {3, 7}, {4, 3}, {5, 4}, {4, 5}}},
	}

	for index, item := range tests {
		assert.Equal(&testing.T{}, item.ans, queensAttacktheKing(item.queens, item.king), index)
	}
}
