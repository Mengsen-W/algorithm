/*
 * @Date: 2022-04-13 09:16:53
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-13 09:32:26
 * @FilePath: /algorithm/380_randomized_set/randomized_set.go
 */

package main

import "math/rand"

type RandomizedSet struct {
	nums    []int
	indices map[int]int
}

func Constructor() RandomizedSet {
	return RandomizedSet{[]int{}, map[int]int{}}
}

func (rs *RandomizedSet) Insert(val int) bool {
	if _, ok := rs.indices[val]; ok {
		return false
	}
	rs.indices[val] = len(rs.nums)
	rs.nums = append(rs.nums, val)
	return true
}

func (rs *RandomizedSet) Remove(val int) bool {
	id, ok := rs.indices[val]
	if !ok {
		return false
	}
	last := len(rs.nums) - 1
	rs.nums[id] = rs.nums[last]
	rs.indices[rs.nums[id]] = id
	rs.nums = rs.nums[:last]
	delete(rs.indices, val)
	return true
}

func (rs *RandomizedSet) GetRandom() int {
	return rs.nums[rand.Intn(len(rs.nums))]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	randomSet := Constructor()
	assert(randomSet.Insert(1))
	assert(!randomSet.Remove(2))
	assert(randomSet.Insert(2))
	assert(randomSet.GetRandom() == 1 || randomSet.GetRandom() == 2)
	assert(randomSet.Remove(1))
	assert(!randomSet.Insert(2))
	assert(randomSet.GetRandom() == 2)
}
