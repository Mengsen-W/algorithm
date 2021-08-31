/*
 * @Date: 2021-08-31 14:23:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-31 14:29:18
 */

package main

import (
	"reflect"
)

func corpFlightBookings(bookings [][]int, n int) []int {
	nums := make([]int, n)
	for _, booking := range bookings {
		nums[booking[0]-1] += booking[2]
		if booking[1] < n {
			nums[booking[1]] -= booking[2]
		}
	}
	for i := 1; i < n; i++ {
		nums[i] += nums[i-1]
	}
	return nums
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		bookings := [][]int{{1, 2, 10}, {2, 3, 20}, {2, 5, 25}}
		n := 5
		answer := []int{10, 55, 45, 25, 25}
		assert(corpFlightBookings(bookings, n), answer)
	}
	{
		bookings := [][]int{{1, 2, 10}, {2, 2, 15}}
		n := 2
		answer := []int{10, 25}
		assert(corpFlightBookings(bookings, n), answer)
	}
}
