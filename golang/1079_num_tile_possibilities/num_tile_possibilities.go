/*
 * @Date: 2023-05-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-19
 * @FilePath: /algorithm/golang/1079_num_tile_possibilities/num_tile_possibilities.go
 */

// Package main ...
package main

func numTilePossibilities(tiles string) int {
	count := make(map[rune]int)
	for _, t := range tiles {
		count[t]++
	}
	var dfs func(i int, count map[rune]int) int
	dfs = func(i int, count map[rune]int) int {
		if i == 0 {
			return 1
		}
		res := 1
		for t := range count {
			if count[t] > 0 {
				count[t]--
				res += dfs(i-1, count)
				count[t]++
			}
		}
		return res
	}
	return dfs(len(tiles), count) - 1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		tiles := "AAB"
		ans := 8
		assert(numTilePossibilities(tiles) == ans)
	}

	{
		tiles := "AAABBC"
		ans := 188
		assert(numTilePossibilities(tiles) == ans)
	}

	{
		tiles := "V"
		ans := 1
		assert(numTilePossibilities(tiles) == ans)
	}
}
