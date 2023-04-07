/*
 * @Date: 2023-04-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-07
 * @FilePath: /algorithm/golang/1040_num_moves_stones_ii/num_moves_stones_ii.go
 */

// Package main ...
package main

import (
	"reflect"
	"sort"
)

func numMovesStonesII(stones []int) []int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	n := len(stones)
	sort.Ints(stones)
	if stones[n-1]-stones[0]+1 == n {
		return []int{0, 0}
	}
	ma := max(stones[n-2]-stones[0]+1, stones[n-1]-stones[1]+1) - (n - 1)
	mi := n
	j := 0
	for i := 0; i < n; i++ {
		for j+1 < n && stones[j+1]-stones[i]+1 <= n {
			j++
		}
		if j-i+1 == n-1 && stones[j]-stones[i]+1 == n-1 {
			mi = min(mi, 2)
		} else {
			mi = min(mi, n-(j-i+1))
		}
	}
	return []int{mi, ma}
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		stones := []int{7, 4, 9}
		ans := []int{1, 2}
		assert(numMovesStonesII(stones), ans)
	}

	{
		stones := []int{6, 5, 4, 3, 10}
		ans := []int{2, 3}
		assert(numMovesStonesII(stones), ans)
	}

	{
		stones := []int{100, 101, 104, 102, 103}
		ans := []int{0, 0}
		assert(numMovesStonesII(stones), ans)
	}
}
