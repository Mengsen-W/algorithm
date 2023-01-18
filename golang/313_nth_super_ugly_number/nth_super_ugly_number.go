/*
 * @Date: 2021-08-09 11:25:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-09 11:39:46
 */

package main

import (
	"container/heap"
	"math"
	"sort"
)

type hp struct{ sort.IntSlice }

func (h *hp) Push(v interface{}) { h.IntSlice = append(h.IntSlice, v.(int)) }

func (h *hp) Pop() interface{} {
	a := h.IntSlice
	v := a[len(a)-1]
	h.IntSlice = a[:len(a)-1]
	return v
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func nthSuperUglyNumber_min_heap(n int, primes []int) (ugly int) {
	seen := map[int]bool{1: true}
	h := &hp{[]int{1}}
	for i := 0; i < n; i++ {
		ugly = heap.Pop(h).(int)
		for _, prime := range primes {
			if next := ugly * prime; !seen[next] {
				seen[next] = true
				heap.Push(h, next)
			}
		}
	}
	return
}

func nthSuperUglyNumber_dp(n int, primes []int) int {
	dp := make([]int, n+1)
	dp[1] = 1
	m := len(primes)
	pointers := make([]int, m)
	for i := range pointers {
		pointers[i] = 1
	}
	for i := 2; i <= n; i++ {
		nums := make([]int, m)
		minNum := math.MaxInt64
		for j, p := range pointers {
			nums[j] = dp[p] * primes[j]
			minNum = min(minNum, nums[j])
		}
		dp[i] = minNum
		for j, num := range nums {
			if minNum == num {
				pointers[j]++
			}
		}
	}
	return dp[n]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		n := 12
		primes := []int{2, 7, 13, 19}
		ans := 32
		assert(nthSuperUglyNumber_min_heap(n, primes) == ans)
		assert(nthSuperUglyNumber_dp(n, primes) == ans)
	}

	{
		n := 1
		primes := []int{2, 3, 5}
		ans := 1
		assert(nthSuperUglyNumber_min_heap(n, primes) == ans)
		assert(nthSuperUglyNumber_dp(n, primes) == ans)
	}

}
