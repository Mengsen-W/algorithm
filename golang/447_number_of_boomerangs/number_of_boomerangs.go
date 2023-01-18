/*
 * @Date: 2021-09-13 08:20:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-13 08:27:24
 */

package main

func numberOfBoomerangs(points [][]int) (ans int) {
	for _, p := range points {
		cnt := map[int]int{}
		for _, q := range points {
			dis := (p[0]-q[0])*(p[0]-q[0]) + (p[1]-q[1])*(p[1]-q[1])
			cnt[dis]++
		}
		for _, m := range cnt {
			ans += m * (m - 1)
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
		points := [][]int{{0, 0}, {1, 0}, {2, 0}}
		ans := 2
		assert(numberOfBoomerangs(points) == ans)
	}
	{
		points := [][]int{{1, 1}, {2, 2}, {3, 3}}
		ans := 2
		assert(numberOfBoomerangs(points) == ans)
	}
	{
		points := [][]int{{1, 1}}
		ans := 0
		assert(numberOfBoomerangs(points) == ans)
	}
}
