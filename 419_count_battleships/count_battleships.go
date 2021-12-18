/*
 * @Date: 2021-12-18 01:04:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-18 01:30:24
 */

package main

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
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}

	assert(countBattleships([][]byte{{'X', '.', '.', 'X'}, {'.', '.', '.', 'X'}, {'.', '.', '.', 'X'}}), 2)
	assert(countBattleships([][]byte{{'.'}}), 0)
}
