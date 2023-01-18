/*
 * @Date: 2021-08-14 13:51:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-14 14:02:27
 */

package main

func unhappyFriends(n int, preferences [][]int, pairs [][]int) (ans int) {
	order := make([][]int, n)
	for i, preference := range preferences {
		order[i] = make([]int, n)
		for j, p := range preference {
			order[i][p] = j
		}
	}
	match := make([]int, n)
	for _, p := range pairs {
		match[p[0]] = p[1]
		match[p[1]] = p[0]
	}

	for x, y := range match {
		index := order[x][y]
		for _, u := range preferences[x][:index] {
			v := match[u]
			if order[u][x] < order[u][v] {
				ans++
				break
			}
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
		n := 4
		preferences := [][]int{{1, 2, 3}, {3, 2, 0}, {3, 1, 0}, {1, 2, 0}}
		pairs := [][]int{{0, 1}, {2, 3}}
		assert(unhappyFriends(n, preferences, pairs) == 2)
	}

	{
		n := 2
		preferences := [][]int{{1}, {0}}
		pairs := [][]int{{1, 0}}
		assert(unhappyFriends(n, preferences, pairs) == 0)
	}

	{
		n := 4
		preferences := [][]int{{1, 3, 2}, {2, 3, 0}, {1, 3, 0}, {0, 2, 1}}
		pairs := [][]int{{1, 3}, {0, 2}}
		assert(unhappyFriends(n, preferences, pairs) == 4)
	}

}
