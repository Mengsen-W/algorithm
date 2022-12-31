/*
 * @Date: 2022-12-31
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-31
 * @FilePath: /algorithm/2037_min_moves_to_seat/min_moves_to_seat.go
 */

package main

import "sort"

func minMovesToSeat(seats, students []int) (ans int) {

	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	sort.Ints(seats)
	sort.Ints(students)
	for i, x := range seats {
		ans += abs(x - students[i])
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		seats := []int{3, 1, 5}
		students := []int{2, 7, 4}
		ans := 4
		assert(minMovesToSeat(seats, students) == ans)
	}

	{
		seats := []int{4, 1, 5, 9}
		students := []int{1, 3, 2, 6}
		ans := 7
		assert(minMovesToSeat(seats, students) == ans)
	}

	{
		seats := []int{2, 2, 6, 6}
		students := []int{1, 3, 2, 6}
		ans := 4
		assert(minMovesToSeat(seats, students) == ans)
	}
}
