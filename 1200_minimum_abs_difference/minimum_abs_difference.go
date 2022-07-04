/*
 * @Date: 2022-07-04
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-04
 * @FilePath: /algorithm/1200_minimum_abs_difference/minimum_abs_difference.go
 */

package main

import (
	"math"
	"reflect"
	"sort"
)

func minimumAbsDifference(arr []int) (ans [][]int) {
	sort.Ints(arr)
	for i, best := 0, math.MaxInt32; i < len(arr)-1; i++ {
		if delta := arr[i+1] - arr[i]; delta < best {
			best = delta
			ans = [][]int{{arr[i], arr[i+1]}}
		} else if delta == best {
			ans = append(ans, []int{arr[i], arr[i+1]})
		}
	}
	return
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		arr := []int{4, 2, 1, 3}
		ans := [][]int{{1, 2}, {2, 3}, {3, 4}}
		assert(minimumAbsDifference(arr), ans)
	}

	{
		arr := []int{1, 3, 6, 10, 15}
		ans := [][]int{{1, 3}}
		assert(minimumAbsDifference(arr), ans)
	}

	{
		arr := []int{3, 8, -10, 23, 19, -4, -14, 27}
		ans := [][]int{{-14, -10}, {19, 23}, {23, 27}}
		assert(minimumAbsDifference(arr), ans)
	}
}
