/*
 * @Date: 2022-10-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-26
 * @FilePath: /algorithm/862_shortest_subarray/shortest_subarray.go
 */

package main

func shortestSubarray(nums []int, k int) int {
	n := len(nums)
	preSumArr := make([]int, n+1)
	for i, num := range nums {
		preSumArr[i+1] = preSumArr[i] + num
	}
	ans := n + 1
	q := []int{}
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	for i, curSum := range preSumArr {
		for len(q) > 0 && curSum-preSumArr[q[0]] >= k {
			ans = min(ans, i-q[0])
			q = q[1:]
		}
		for len(q) > 0 && preSumArr[q[len(q)-1]] >= curSum {
			q = q[:len(q)-1]
		}
		q = append(q, i)
	}
	if ans < n+1 {
		return ans
	}
	return -1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{1}
		k := 1
		ans := 1
		assert(shortestSubarray(nums, k) == ans)
	}

	{
		nums := []int{1, 2}
		k := 4
		ans := -1
		assert(shortestSubarray(nums, k) == ans)
	}

	{
		nums := []int{2, -1, 2}
		k := 3
		ans := 3
		assert(shortestSubarray(nums, k) == ans)
	}
}
