/*
 * @Date: 2022-07-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-24
 * @FilePath: /algorithm/1184_distance_between_bus_stops/distance_between_bus_stops.go
 */

package main

func distanceBetweenBusStops(distance []int, start, destination int) int {
	if start > destination {
		start, destination = destination, start
	}
	sum1, sum2 := 0, 0
	for i, d := range distance {
		if start <= i && i < destination {
			sum1 += d
		} else {
			sum2 += d
		}
	}
	return func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}(sum1, sum2)
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}
	{
		distance := []int{1, 2, 3, 4}
		start := 0
		destination := 1
		assert(distanceBetweenBusStops(distance, start, destination), 1)
	}

	{
		distance := []int{1, 2, 3, 4}
		start := 0
		destination := 2
		assert(distanceBetweenBusStops(distance, start, destination), 3)
	}

	{
		distance := []int{1, 2, 3, 4}
		start := 0
		destination := 3
		assert(distanceBetweenBusStops(distance, start, destination), 4)
	}
}
