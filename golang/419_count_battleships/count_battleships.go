/*
 * @Date: 2021-12-18 01:04:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-18 01:30:24
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countBattleships(board [][]byte) (ans int) {
	for i, row := range board {
		for j, ch := range row {
			if ch == 'X' && !(i > 0 && board[i-1][j] == 'X' || j > 0 && board[i][j-1] == 'X') {
				ans++
			}
		}
	}
	return
}

func main() {
	tests := []struct {
		board [][]byte
		ans   int
	}{
		{[][]byte{{'X', '.', '.', 'X'}, {'.', '.', '.', 'X'}, {'.', '.', '.', 'X'}}, 2},
		{[][]byte{{'.'}}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countBattleships(test.board), index)
	}
}
