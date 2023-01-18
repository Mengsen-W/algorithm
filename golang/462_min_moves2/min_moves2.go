/*
 * @Date: 2022-05-20 21:43:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-20 21:50:59
 * @FilePath: /algorithm/462_min_moves2/min_moves2.go
 */

package main

import (
	"math/rand"
	"time"
)

func partition(a []int, l, r int) int {
	x := a[r]
	i := l - 1
	for j := l; j < r; j++ {
		if a[j] <= x {
			i++
			a[i], a[j] = a[j], a[i]
		}
	}
	a[i+1], a[r] = a[r], a[i+1]
	return i + 1
}

func randomPartition(a []int, l, r int) int {
	i := rand.Intn(r-l+1) + l
	a[i], a[r] = a[r], a[i]
	return partition(a, l, r)
}

func quickSelect(a []int, l, r, index int) int {
	q := randomPartition(a, l, r)
	if q == index {
		return a[q]
	}
	if q < index {
		return quickSelect(a, q+1, r, index)
	}
	return quickSelect(a, l, q-1, index)
}

func minMoves2(nums []int) (ans int) {
	rand.Seed(time.Now().UnixNano())
	x := quickSelect(nums, 0, len(nums)-1, len(nums)/2)
	for _, num := range nums {
		ans += abs(num - x)
	}
	return
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(minMoves2([]int{1, 2, 3}) == 2)
	assert(minMoves2([]int{1, 10, 2, 9}) == 16)
}
