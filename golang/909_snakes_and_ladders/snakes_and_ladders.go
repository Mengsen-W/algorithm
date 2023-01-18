/*
 * @Date: 2021-06-27 10:07:47
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-27 10:21:05
 */

package main

func id2rc(id, n int) (r, c int) {
	r, c = (id-1)/n, (id-1)%n
	if r%2 == 1 {
		c = n - 1 - c
	}
	r = n - 1 - r
	return
}

func snakesAndLadders(board [][]int) int {
	n := len(board)
	vis := make([]bool, n*n+1)
	type pair struct{ id, step int }
	q := []pair{{1, 0}}
	for len(q) > 0 {
		p := q[0]
		q = q[1:]
		for i := 1; i <= 6; i++ {
			nxt := p.id + i
			if nxt > n*n { // 超出边界
				break
			}
			r, c := id2rc(nxt, n) // 得到下一步的行列
			if board[r][c] > 0 {  // 存在蛇或梯子
				nxt = board[r][c]
			}
			if nxt == n*n { // 到达终点
				return p.step + 1
			}
			if !vis[nxt] {
				vis[nxt] = true
				q = append(q, pair{nxt, p.step + 1}) // 扩展新状态
			}
		}
	}
	return -1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	board := [][]int{{-1, -1, -1, -1, -1, -1}, {-1, -1, -1, -1, -1, -1},
		{-1, -1, -1, -1, -1, -1}, {-1, 35, -1, -1, 13, -1},
		{-1, -1, -1, -1, -1, -1}, {-1, 15, -1, -1, -1, -1}}
	assert(snakesAndLadders(board) == 4)

}
