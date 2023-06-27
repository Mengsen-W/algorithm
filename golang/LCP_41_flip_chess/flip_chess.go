/*
 * @Date: 2023-06-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-21
 * @FilePath: /algorithm/golang/LCP_41_flip_chess/flip_chess.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func flipChess(chessboard []string) (ans int) {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	m, n := len(chessboard), len(chessboard[0])
	bfs := func(i, j int) int {
		q := [][2]int{{i, j}}
		g := make([][]byte, m)
		for i := range g {
			g[i] = make([]byte, n)
			copy(g[i], []byte(chessboard[i]))
		}
		cnt := 0
		for len(q) > 0 {
			p := q[0]
			q = q[1:]
			i, j = p[0], p[1]
			for a := -1; a <= 1; a++ {
				for b := -1; b <= 1; b++ {
					if a == 0 && b == 0 {
						continue
					}
					x, y := i+a, j+b
					for x >= 0 && x < m && y >= 0 && y < n && g[x][y] == 'O' {
						x, y = x+a, y+b
					}
					if x >= 0 && x < m && y >= 0 && y < n && g[x][y] == 'X' {
						x -= a
						y -= b
						cnt += max(abs(x-i), abs(y-j))
						for x != i || y != j {
							g[x][y] = 'X'
							q = append(q, [2]int{x, y})
							x -= a
							y -= b
						}
					}
				}
			}
		}
		return cnt
	}
	for i, row := range chessboard {
		for j, c := range row {
			if c == '.' {
				ans = max(ans, bfs(i, j))
			}
		}
	}
	return
}

func main() {
	{
		chessboard := []string{"....X.", "....X.", "XOOO..", "......", "......"}
		ans := 3
		assert.Equal(&testing.B{}, flipChess(chessboard), ans)
	}

	{
		chessboard := []string{".X.", ".O.", "XO."}
		ans := 2
		assert.Equal(&testing.B{}, flipChess(chessboard), ans)
	}

	{
		chessboard := []string{".......", ".......", ".......", "X......", ".O.....", "..O....", "....OOX"}
		ans := 4
		assert.Equal(&testing.B{}, flipChess(chessboard), ans)
	}
}
