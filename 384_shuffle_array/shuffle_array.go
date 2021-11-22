/*
 * @Date: 2021-11-22 02:11:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-22 02:19:01
 */

package main

import (
	"fmt"
	"math/rand"
)

type Solution struct {
	nums, original []int
}

func Constructor(nums []int) Solution {
	return Solution{nums, append([]int(nil), nums...)}
}

func (s *Solution) Reset() []int {
	copy(s.nums, s.original)
	return s.nums
}

func (s *Solution) Shuffle() []int {
	n := len(s.nums)
	for i := range s.nums {
		j := i + rand.Intn(n-i)
		s.nums[i], s.nums[j] = s.nums[j], s.nums[i]
	}
	return s.nums
}

func main() {
	{
		s := Constructor([]int{1, 2, 3})
		fmt.Println(s.Shuffle())
		fmt.Println(s.Reset())
		fmt.Println(s.Shuffle())
	}
}
