/*
 * @Date: 2022-09-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-19
 * @FilePath: /algorithm/1636_frequency_sort/frequency_sort.go
 */

package main

import (
	"reflect"
	"sort"
)

func frequencySort(nums []int) []int {
	cnt := map[int]int{}
	for _, x := range nums {
		cnt[x]++
	}
	sort.Slice(nums, func(i, j int) bool {
		a, b := nums[i], nums[j]
		return cnt[a] < cnt[b] || cnt[a] == cnt[b] && a > b
	})
	return nums
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		nums := []int{1, 1, 2, 2, 2, 3}
		ans := []int{3, 1, 1, 2, 2, 2}
		assert(frequencySort(nums), ans)
	}

	{
		nums := []int{2, 3, 1, 3, 2}
		ans := []int{1, 3, 3, 2, 2}
		assert(frequencySort(nums), ans)
	}

	{
		nums := []int{-1, 1, -6, 4, 5, -6, 1, 4, 1}
		ans := []int{5, -1, 4, 4, -6, -6, 1, 1, 1}
		assert(frequencySort(nums), ans)
	}
}
