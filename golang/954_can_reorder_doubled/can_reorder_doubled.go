/*
 * @Date: 2022-03-31 22:14:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-31 22:21:05
 * @FilePath: /algorithm/954_can_reorder_doubled/can_reorder_doubled.go
 */

package main

import "sort"

func canReorderDoubled(arr []int) bool {
	cnt := make(map[int]int, len(arr))
	for _, x := range arr {
		cnt[x]++
	}
	if cnt[0]%2 == 1 {
		return false
	}

	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	vals := make([]int, 0, len(cnt))
	for x := range cnt {
		vals = append(vals, x)
	}
	sort.Slice(vals, func(i, j int) bool { return abs(vals[i]) < abs(vals[j]) })

	for _, x := range vals {
		if cnt[2*x] < cnt[x] { // 无法找到足够的 2x 与 x 配对
			return false
		}
		cnt[2*x] -= cnt[x]
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(canReorderDoubled([]int{3, 1, 3, 6}) == false)
	assert(canReorderDoubled([]int{2, 1, 2, 6}) == false)
	assert(canReorderDoubled([]int{4, -2, 2, -4}) == true)
}
