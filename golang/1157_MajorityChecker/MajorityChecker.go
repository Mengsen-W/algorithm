/*
 * @Date: 2023-04-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-16
 * @FilePath: /algorithm/golang/1157_MajorityChecker/MajorityChecker.go
 */

// Package main ...
package main

import (
	"math/rand"
	"sort"
	"time"
)

type MajorityChecker struct {
	arr []int
	loc map[int][]int
}

func Constructor(arr []int) MajorityChecker {
	rand.Seed(time.Now().UnixNano())
	loc := map[int][]int{}
	for i, x := range arr {
		loc[x] = append(loc[x], i)
	}
	return MajorityChecker{arr, loc}
}

func (mc *MajorityChecker) Query(left, right, threshold int) int {
	length := right - left + 1
	for i := 0; i < 20; i++ {
		x := mc.arr[rand.Intn(right-left+1)+left]
		pos := mc.loc[x]
		occ := sort.SearchInts(pos, right+1) - sort.SearchInts(pos, left)
		if occ >= threshold {
			return x
		}
		if occ*2 >= length {
			break
		}
	}
	return -1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	arr := []int{1, 1, 2, 2, 1, 1}
	m := Constructor(arr)
	assert(m.Query(0, 5, 4) == 1)
	assert(m.Query(0, 3, 3) == -1)
	assert(m.Query(2, 3, 2) == 2)
}
