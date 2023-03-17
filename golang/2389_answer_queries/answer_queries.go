/*
 * @Date: 2023-03-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-17
 * @FilePath: /algorithm/golang/2389_answer_queries/answer_queries.go
 */

// Package ...
package main

import (
	"reflect"
	"sort"
)

func answerQueries(nums []int, queries []int) []int {
	sort.Ints(nums)
	n := len(nums)
	f := make([]int, n)
	f[0] = nums[0]
	for i := 1; i < n; i++ {
		f[i] = f[i-1] + nums[i]
	}
	ans := []int{}
	for _, q := range queries {
		idx := sort.SearchInts(f, q+1)
		ans = append(ans, idx)
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		nums := []int{4, 5, 2, 1}
		queries := []int{3, 10, 21}
		ans := []int{2, 3, 4}
		assert(answerQueries(nums, queries), ans)
	}

	{
		nums := []int{2, 3, 4, 5}
		queries := []int{1}
		ans := []int{0}
		assert(answerQueries(nums, queries), ans)
	}
}
