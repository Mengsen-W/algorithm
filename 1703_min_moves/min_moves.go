/*
 * @Date: 2022-12-18
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-18
 * @FilePath: /algorithm/1703_min_moves/min_moves.go
 */

package main

import "math"

func minMoves(nums []int, k int) int {
	min := func(a, b int) int {
		if a <= b {
			return a
		} else {
			return b
		}
	}
	g := []int{}
	preSum := []int{0}

	for i := 0; i < len(nums); i++ {
		if nums[i] == 1 {
			g = append(g, i-len(g))
			preSum = append(preSum, (preSum[len(preSum)-1] + g[len(g)-1]))
		}
	}

	m, res := len(g), math.MaxInt

	for i := 0; i <= m-k; i++ {
		mid := i + k/2
		res = min(res, (1-k%2)*g[mid]+(preSum[i+k]-preSum[mid+1])-(preSum[mid]-preSum[i]))
	}
	return res
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{1, 0, 0, 1, 0, 1}
		k := 2
		ans := 1
		assert(minMoves(nums, k) == ans)
	}

	{
		nums := []int{1, 0, 0, 0, 0, 0, 1, 1}
		k := 3
		ans := 5
		assert(minMoves(nums, k) == ans)
	}

	{
		nums := []int{1, 1, 0, 1}
		k := 2
		ans := 0
		assert(minMoves(nums, k) == ans)
	}
}
