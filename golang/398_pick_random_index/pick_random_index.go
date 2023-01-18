/*
 * @Date: 2022-04-25 09:28:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-25 09:34:50
 * @FilePath: /algorithm/398_pick_random_index/pick_random_index.go
 */

package main

import "math/rand"

type Solution []int

func Constructor(nums []int) Solution {
	return nums
}

func (nums Solution) Pick(target int) (ans int) {
	cnt := 0
	for i, num := range nums {
		if num == target {
			cnt++ // 第 cnt 次遇到 target
			if rand.Intn(cnt) == 0 {
				ans = i
			}
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	s := Constructor([]int{1, 2, 3, 3, 3})
	assert(s.Pick(1) == 0)
	assert(s.Pick(2) == 1)
	assert(s.Pick(3) == 2 || s.Pick(3) == 3 || s.Pick(3) == 4)
}
