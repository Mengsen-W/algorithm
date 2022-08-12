/*
 * @Date: 2022-08-12
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-12
 * @FilePath: /algorithm/1282_group_the_people/group_the_people.go
 */

package main

import "reflect"

func groupThePeople(groupSizes []int) (ans [][]int) {
	groups := map[int][]int{}
	for i, size := range groupSizes {
		groups[size] = append(groups[size], i)
	}
	for size, people := range groups {
		for i := 0; i < len(people); i += size {
			ans = append(ans, people[i:i+size])
		}
	}
	return
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(groupThePeople([]int{3, 3, 3, 3, 3, 1, 3}), [][]int{{5}, {0, 1, 2}, {3, 4, 6}})
	assert(groupThePeople([]int{2, 1, 3, 3, 3, 2}), [][]int{{2, 3, 4}, {1}, {0, 5}})
}
