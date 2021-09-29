/*
 * @Date: 2021-09-29 09:46:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-29 10:08:47
 */

package main

func findMinMoves(machines []int) (ans int) {
	tot := 0
	for _, v := range machines {
		tot += v
	}
	n := len(machines)
	if tot%n > 0 {
		return -1
	}
	avg := tot / n
	sum := 0
	for _, num := range machines {
		num -= avg
		sum += num
		ans = max(ans, max(abs(sum), num))
	}
	return
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
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
	{
		machines := []int{1, 0, 5}
		assert(findMinMoves(machines) == 3)
	}
	{
		machines := []int{0, 3, 0}
		assert(findMinMoves(machines) == 2)
	}
	{
		machines := []int{0, 2, 0}
		assert(findMinMoves(machines) == -1)
	}
}
