/*
 * @Date: 2023-02-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-16
 * @FilePath: /algorithm/golang/2341_number_of_pairs/number_of_pairs.go
 */

package main

import "reflect"

func numberOfPairs(nums []int) []int {
	cnt := map[int]bool{}
	res := 0
	for _, num := range nums {
		cnt[num] = !cnt[num]
		if !cnt[num] {
			res++
		}
	}
	return []int{res, len(nums) - 2*res}
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		nums := []int{1, 3, 2, 1, 3, 2, 2}
		ans := []int{3, 1}
		assert(numberOfPairs(nums), ans)
	}

	{
		nums := []int{1, 1}
		ans := []int{1, 0}
		assert(numberOfPairs(nums), ans)
	}

	{
		nums := []int{0}
		ans := []int{0, 1}
		assert(numberOfPairs(nums), ans)
	}
}
