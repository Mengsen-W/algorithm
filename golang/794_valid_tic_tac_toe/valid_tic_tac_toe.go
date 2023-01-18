/*
 * @Date: 2021-12-09 05:48:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-09 05:57:00
 */

package main

import "strings"

func win(board []string, p byte) bool {
	for i := 0; i < 3; i++ {
		if board[i][0] == p && board[i][1] == p && board[i][2] == p ||
			board[0][i] == p && board[1][i] == p && board[2][i] == p {
			return true
		}
	}
	return board[0][0] == p && board[1][1] == p && board[2][2] == p ||
		board[0][2] == p && board[1][1] == p && board[2][0] == p
}

func validTicTacToe(board []string) bool {
	oCount, xCount := 0, 0
	for _, row := range board {
		oCount += strings.Count(row, "O")
		xCount += strings.Count(row, "X")
	}
	return !(oCount != xCount && oCount != xCount-1 ||
		oCount != xCount && win(board, 'O') ||
		oCount != xCount-1 && win(board, 'X'))
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(!validTicTacToe([]string{"O  ", "   ", "   "}))
	assert(!validTicTacToe([]string{"XOX", " X ", "   "}))
	assert(!validTicTacToe([]string{"XXX", "   ", "OOO"}))
	assert(validTicTacToe([]string{"XOX", "O O", "XOX"}))
}
