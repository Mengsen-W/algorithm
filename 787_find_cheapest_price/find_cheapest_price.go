/*
 * @Date: 2021-08-24 09:45:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-24 10:36:04
 */

package main

func findCheapestPrice(n int, flights [][]int, src int, dst int, k int) int {
	const inf = 10000*101 + 1
	f := make([][]int, k+2)
	for i := range f {
		f[i] = make([]int, n)
		for j := range f[i] {
			f[i][j] = inf
		}
	}
	f[0][src] = 0
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	for t := 1; t <= k+1; t++ {
		for _, flight := range flights {
			j, i, cost := flight[0], flight[1], flight[2]
			f[t][i] = min(f[t][i], f[t-1][j]+cost)
		}
	}
	ans := inf

	for t := 1; t <= k+1; t++ {
		ans = min(ans, f[t][dst])
	}
	if ans == inf {
		ans = -1
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		n := 3
		edges := [][]int{{0, 1, 100}, {1, 2, 100}, {0, 2, 500}}
		src := 0
		dst := 2
		k := 1
		assert(findCheapestPrice(n, edges, src, dst, k) == 200)
	}
	{
		n := 3
		edges := [][]int{{0, 1, 100}, {1, 2, 100}, {0, 2, 500}}
		src := 0
		dst := 2
		k := 0
		assert(findCheapestPrice(n, edges, src, dst, k) == 500)
	}
}
