/*
 * @Date: 2022-08-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-23
 * @FilePath: /algorithm/782_moves_to_chessboard/moves_to_chessboard.go
 */

package main

import "math/bits"

func getMoves(mask uint, count, n int) int {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	ones := bits.OnesCount(mask)
	if n&1 > 0 {
		// 如果 n 为奇数，则每一行中 1 与 0 的数目相差为 1，且满足相邻行交替
		if abs(n-2*ones) != 1 || abs(n-2*count) != 1 {
			return -1
		}
		if ones == n>>1 {
			// 偶数位变为 1 的最小交换次数
			return n/2 - bits.OnesCount(mask&0xAAAAAAAA)
		} else {
			// 奇数位变为 1 的最小交换次数
			return (n+1)/2 - bits.OnesCount(mask&0x55555555)
		}
	} else {
		// 如果 n 为偶数，则每一行中 1 与 0 的数目相等，且满足相邻行交替
		if ones != n>>1 || count != n>>1 {
			return -1
		}
		// 偶数位变为 1 的最小交换次数
		count0 := n/2 - bits.OnesCount(mask&0xAAAAAAAA)
		// 奇数位变为 1 的最小交换次数
		count1 := n/2 - bits.OnesCount(mask&0x55555555)
		return min(count0, count1)
	}
}

func movesToChessboard(board [][]int) int {
	n := len(board)
	// 棋盘的第一行与第一列
	rowMask, colMask := 0, 0
	for i := 0; i < n; i++ {
		rowMask |= board[0][i] << i
		colMask |= board[i][0] << i
	}
	reverseRowMask := 1<<n - 1 ^ rowMask
	reverseColMask := 1<<n - 1 ^ colMask
	rowCnt, colCnt := 0, 0
	for i := 0; i < n; i++ {
		currRowMask, currColMask := 0, 0
		for j := 0; j < n; j++ {
			currRowMask |= board[i][j] << j
			currColMask |= board[j][i] << j
		}
		if currRowMask != rowMask && currRowMask != reverseRowMask || // 检测每一行的状态是否合法
			currColMask != colMask && currColMask != reverseColMask { // 检测每一列的状态是否合法
			return -1
		}
		if currRowMask == rowMask {
			rowCnt++ // 记录与第一行相同的行数
		}
		if currColMask == colMask {
			colCnt++ // 记录与第一列相同的列数
		}
	}
	rowMoves := getMoves(uint(rowMask), rowCnt, n)
	colMoves := getMoves(uint(colMask), colCnt, n)
	if rowMoves == -1 || colMoves == -1 {
		return -1
	}
	return rowMoves + colMoves
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		board := [][]int{{0, 1, 1, 0}, {0, 1, 1, 0}, {1, 0, 0, 1}, {1, 0, 0, 1}}
		ans := 2
		assert(movesToChessboard(board) == ans)
	}
	{
		board := [][]int{{0, 1}, {1, 0}}
		ans := 0
		assert(movesToChessboard(board) == ans)
	}
	{
		board := [][]int{{1, 0}, {1, 0}}
		ans := -1
		assert(movesToChessboard(board) == ans)
	}
}
