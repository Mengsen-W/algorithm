/*
 * @Date: 2021-05-25 09:34:19
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-25 09:42:19
 */

package main

import "math"

func minChanges(nums []int, k int) int {
	min := func(a ...int) int {
		res := a[0]
		for _, v := range a[1:] {
			if v < res {
				res = v
			}
		}
		return res
	}
	const maxX = 1 << 10          // x 的范围为 [0, 2^10)
	const inf = math.MaxInt32 / 2 // 极大值，为了防止整数溢出

	n := len(nums)
	f := make([]int, maxX)
	for i := range f {
		f[i] = inf
	}
	// 边界条件 f(-1,0)=0
	f[0] = 0

	for i := 0; i < k; i++ {
		// 第 i 个组的哈希映射
		cnt := map[int]int{}
		size := 0
		for j := i; j < n; j += k {
			cnt[nums[j]]++
			size++
		}

		// 求出 t2
		t2min := min(f...)

		g := make([]int, maxX)
		for j := range g {
			g[j] = t2min
		}
		for mask := range g {
			// t1 则需要枚举 x 才能求出
			for x, cntX := range cnt {
				g[mask] = min(g[mask], f[mask^x]-cntX)
			}
		}

		// 别忘了加上 size
		for j := range g {
			g[j] += size
		}
		f = g
	}

	return f[0]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	assert(minChanges([]int{1, 2, 0, 3, 0}, 1) == 3)
	assert(minChanges([]int{3, 4, 5, 2, 1, 7, 3, 4, 7}, 3) == 3)
	assert(minChanges([]int{1, 2, 4, 1, 2, 5, 1, 2, 6}, 3) == 3)
}
