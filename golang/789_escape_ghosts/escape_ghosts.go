/*
 * @Date: 2021-08-22 13:17:53
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-22 13:29:00
 */

package main

func escapeGhosts(ghosts [][]int, target []int) bool {
	source := []int{0, 0}
	distance := manhattanDistance(source, target)
	for _, ghost := range ghosts {
		if manhattanDistance(ghost, target) <= distance {
			return false
		}
	}
	return true
}

func manhattanDistance(point1, point2 []int) int {
	return abs(point1[0]-point2[0]) + abs(point1[1]-point2[1])
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		ghosts := [][]int{{1, 0}, {0, 3}}
		target := []int{0, 1}
		assert(escapeGhosts(ghosts, target))
	}
	{
		ghosts := [][]int{{1, 0}}
		target := []int{2, 0}
		assert(!escapeGhosts(ghosts, target))
	}
	{
		ghosts := [][]int{{2, 0}}
		target := []int{1, 0}
		assert(!escapeGhosts(ghosts, target))
	}
	{
		ghosts := [][]int{{5, 0}, {-10, -2}, {0, -5}, {-2, -2}, {-7, 1}}
		target := []int{7, 7}
		assert(!escapeGhosts(ghosts, target))
	}
	{
		ghosts := [][]int{{-1, 0}, {0, 1}, {-1, 0}, {0, 1}, {-1, 0}}
		target := []int{0, 0}
		assert(escapeGhosts(ghosts, target))
	}
}
