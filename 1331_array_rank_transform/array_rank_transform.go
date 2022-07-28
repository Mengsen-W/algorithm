/*
 * @Date: 2022-07-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-28
 * @FilePath: /algorithm/1331_array_rank_transform/array_rank_transform.go
 */

package main

import (
	"reflect"
	"sort"
)

func arrayRankTransform(arr []int) []int {
	a := append([]int{}, arr...)
	sort.Ints(a)
	ranks := map[int]int{}
	for _, v := range a {
		if _, ok := ranks[v]; !ok {
			ranks[v] = len(ranks) + 1
		}
	}
	for i, v := range arr {
		arr[i] = ranks[v]
	}
	return arr
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		arr := []int{40, 10, 20, 30}
		ans := []int{4, 1, 2, 3}
		assert(arrayRankTransform(arr), ans)
	}

	{
		arr := []int{100, 100, 100}
		ans := []int{1, 1, 1}
		assert(arrayRankTransform(arr), ans)
	}

	{
		arr := []int{37, 12, 28, 9, 100, 56, 80, 5, 12}
		ans := []int{5, 3, 4, 2, 8, 6, 7, 1, 3}
		assert(arrayRankTransform(arr), ans)
	}
}
