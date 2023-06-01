/*
 * @Date: 2023-06-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-01
 * @FilePath: /algorithm/golang/2517_maximum_tastiness/maximum_tastiness.go
 */

// Package main ...
package main

import (
	"math"
	"sort"
)

func maximumTastiness(price []int, k int) int {
	sort.Ints(price)
	left, right := 0, price[len(price)-1]-price[0]
	for left < right {
		mid := (left + right + 1) / 2
		if check(price, k, mid) {
			left = mid
		} else {
			right = mid - 1
		}
	}
	return left
}

func check(price []int, k int, tastiness int) bool {
	prev := int(math.Inf(-1)) >> 1
	cnt := 0
	for _, p := range price {
		if p-prev >= tastiness {
			cnt++
			prev = p
		}
	}
	return cnt >= k
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		price := []int{13, 5, 1, 8, 21, 2}
		k := 3
		ans := 8
		assert(maximumTastiness(price, k) == ans)
	}

	{
		price := []int{1, 3, 1}
		k := 2
		ans := 2
		assert(maximumTastiness(price, k) == ans)
	}

	{
		price := []int{7, 7, 7, 7}
		k := 2
		ans := 0
		assert(maximumTastiness(price, k) == ans)
	}
}
