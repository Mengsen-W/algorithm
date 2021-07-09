/*
 * @Date: 2021-07-09 09:12:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-09 09:29:41
 */

package main

type Pair struct {
	first  int
	second int
}

func majorityElement(nums []int) int {
	ret := Pair{0, 0}
	for i := 0; i < len(nums); i++ {
		if ret.first == nums[i] {
			ret.second++
		} else if ret.second == 0 {
			ret.first = nums[i]
			ret.second = 1
		} else {
			ret.second--
		}
	}
	count := 0
	for _, s := range nums {
		if s == ret.first {
			count++
		}
	}
	if count > len(nums)/2 {
		return ret.first
	} else {
		return -1
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{1, 2, 5, 9, 5, 9, 5, 5, 5}
		assert(majorityElement(nums) == 5)
	}
	{
		nums := []int{3, 2}
		assert(majorityElement(nums) == -1)
	}
	{
		nums := []int{2, 2, 1, 1, 1, 2, 2}
		assert(majorityElement(nums) == 2)
	}
}
