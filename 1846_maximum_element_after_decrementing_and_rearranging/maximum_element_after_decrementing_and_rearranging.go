/*
 * @Date: 2021-07-15 09:17:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-15 09:25:07
 */

package main

func maximumElementAfterDecrementingAndRearranging(arr []int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	n := len(arr)
	cnt := make([]int, n+1)
	for _, v := range arr {
		cnt[min(v, n)]++
	}
	miss := 0
	for _, c := range cnt[1:] {
		if c == 0 {
			miss++
		} else {
			miss -= min(c-1, miss) // miss 不会小于 0，故至多减去 miss 个元素
		}
	}
	return n - miss
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		arr := []int{2, 2, 1, 2, 1}
		ans := 2
		assert(maximumElementAfterDecrementingAndRearranging(arr) == ans)
	}
	{
		arr := []int{100, 1, 1000}
		ans := 3
		assert(maximumElementAfterDecrementingAndRearranging(arr) == ans)
	}
	{
		arr := []int{1, 2, 3, 4, 5}
		ans := 5
		assert(maximumElementAfterDecrementingAndRearranging(arr) == ans)
	}
}
