/*
 * @Date: 2022-08-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-04
 * @FilePath: /algorithm/1403_min_subsequence/min_subsequence.go
 */

package main

import (
	"reflect"
	"sort"
)

func minSubsequence(nums []int) []int {
	sort.Sort(sort.Reverse(sort.IntSlice(nums)))
	tot := 0
	for _, num := range nums {
		tot += num
	}
	for i, sum := 0, 0; ; i++ {
		sum += nums[i]
		if sum > tot-sum {
			return nums[:i+1]
		}
	}
}

func main() {
	assert := func(a, b []int) {
		if reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		nums := []int{4, 3, 10, 9, 8}
		ans := []int{9, 10}
		assert(minSubsequence(nums), ans)
	}

	{
		nums := []int{4, 4, 7, 6, 7}
		ans := []int{7, 6, 7}
		assert(minSubsequence(nums), ans)
	}
}
