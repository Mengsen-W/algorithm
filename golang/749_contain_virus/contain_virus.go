/*
 * @Date: 2022-07-18
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-18
 * @FilePath: /algorithm/749_contain_virus/contain_virus.go
 */

package main

type pair struct{ x, y int }

var dirs = []pair{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

func containVirus(isInfected [][]int) (ans int) {
	m, n := len(isInfected), len(isInfected[0])
	for {
		neighbors := []map[pair]struct{}{}
		firewalls := []int{}
		for i, row := range isInfected {
			for j, infected := range row {
				if infected != 1 {
					continue
				}
				q := []pair{{i, j}}
				neighbor := map[pair]struct{}{}
				firewall, idx := 0, len(neighbors)+1
				row[j] = -idx
				for len(q) > 0 {
					p := q[0]
					q = q[1:]
					for _, d := range dirs {
						if x, y := p.x+d.x, p.y+d.y; 0 <= x && x < m && 0 <= y && y < n {
							if isInfected[x][y] == 1 {
								q = append(q, pair{x, y})
								isInfected[x][y] = -idx
							} else if isInfected[x][y] == 0 {
								firewall++
								neighbor[pair{x, y}] = struct{}{}
							}
						}
					}
				}
				neighbors = append(neighbors, neighbor)
				firewalls = append(firewalls, firewall)
			}
		}

		if len(neighbors) == 0 {
			break
		}

		idx := 0
		for i := 1; i < len(neighbors); i++ {
			if len(neighbors[i]) > len(neighbors[idx]) {
				idx = i
			}
		}

		ans += firewalls[idx]
		for _, row := range isInfected {
			for j, v := range row {
				if v < 0 {
					if v != -idx-1 {
						row[j] = 1
					} else {
						row[j] = 2
					}
				}
			}
		}

		for i, neighbor := range neighbors {
			if i != idx {
				for p := range neighbor {
					isInfected[p.x][p.y] = 1
				}
			}
		}

		if len(neighbors) == 1 {
			break
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		isInfected := [][]int{{0, 1, 0, 0, 0, 0, 0, 1}, {0, 1, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 0, 0, 0}}
		assert(containVirus(isInfected) == 10)
	}

	{
		isInfected := [][]int{{1, 1, 1}, {1, 0, 1}, {1, 1, 1}}
		assert(containVirus(isInfected) == 4)
	}

	{
		isInfected := [][]int{{1, 1, 1, 0, 0, 0, 0, 0, 0}, {1, 0, 1, 0, 1, 1, 1, 1, 1}, {1, 1, 1, 0, 0, 0, 0, 0, 0}}
		assert(containVirus(isInfected) == 13)
	}
}
