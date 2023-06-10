/*
 * @Date: 2023-06-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-10
 * @FilePath: /algorithm/golang/1170_num_smaller_by_frequency/num_smaller_by_frequency.go
 */

// Package main ...
package main

import "reflect"

func numSmallerByFrequency(queries []string, words []string) []int {
	f := func(s string) int {
		cnt := 0
		ch := 'z'
		for _, c := range s {
			if c < ch {
				ch = c
				cnt = 1
			} else if c == ch {
				cnt++
			}
		}
		return cnt
	}
	count := make([]int, 12)
	for _, s := range words {
		count[f(s)]++
	}
	for i := 9; i >= 1; i-- {
		count[i] += count[i+1]
	}
	res := make([]int, len(queries))
	for i, s := range queries {
		res[i] = count[f(s)+1]
	}
	return res
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		queries := []string{"cbd"}
		words := []string{"zaaaz"}
		ans := []int{1}
		assert(numSmallerByFrequency(queries, words), ans)
	}

	{
		queries := []string{"bbb", "cc"}
		words := []string{"a", "aa", "aaa", "aaaa"}
		ans := []int{1, 2}
		assert(numSmallerByFrequency(queries, words), ans)
	}
}
