/*
 * @Date: 2021-11-03 01:11:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-03 01:42:08
 */

package main

func trapRainWater(heightMap [][]int) (ans int) {
	m, n := len(heightMap), len(heightMap[0])
	maxHeight := 0
	for _, row := range heightMap {
		for _, h := range row {
			maxHeight = max(maxHeight, h)
		}
	}

	water := make([][]int, m)
	for i := range water {
		water[i] = make([]int, n)
		for j := range water[i] {
			water[i][j] = maxHeight
		}
	}
	type pair struct{ x, y int }
	q := []pair{}
	for i, row := range heightMap {
		for j, h := range row {
			if (i == 0 || i == m-1 || j == 0 || j == n-1) && h < water[i][j] {
				water[i][j] = h
				q = append(q, pair{i, j})
			}
		}
	}

	dirs := []int{-1, 0, 1, 0, -1}
	for len(q) > 0 {
		p := q[0]
		q = q[1:]
		x, y := p.x, p.y
		for i := 0; i < 4; i++ {
			nx, ny := x+dirs[i], y+dirs[i+1]
			if 0 <= nx && nx < m && 0 <= ny && ny < n && water[nx][ny] > water[x][y] && water[nx][ny] > heightMap[nx][ny] {
				water[nx][ny] = max(water[x][y], heightMap[nx][ny])
				q = append(q, pair{nx, ny})
			}
		}
	}

	for i, row := range heightMap {
		for j, h := range row {
			ans += water[i][j] - h
		}
	}
	return
}

func max(a, b int) int {
	if b > a {
		return b
	}
	return a
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(trapRainWater([][]int{{1, 4, 3, 1, 3, 2}, {3, 2, 1, 3, 2, 4}, {2, 3, 3, 2, 3, 1}}) == 4)
	assert(trapRainWater([][]int{{3, 3, 3, 3, 3}, {3, 2, 2, 2, 3}, {3, 2, 1, 2, 3}, {3, 2, 2, 2, 3}, {3, 3, 3, 3, 3}}) == 10)
}
