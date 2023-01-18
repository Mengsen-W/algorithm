/*
 * @Date: 2022-11-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-15
 * @FilePath: /algorithm/507_maximum_units/maximum_units.go
 */

package main

import "sort"

func maximumUnits(boxTypes [][]int, truckSize int) (ans int) {
	sort.Slice(boxTypes, func(i, j int) bool { return boxTypes[i][1] > boxTypes[j][1] })
	for _, p := range boxTypes {
		if p[0] >= truckSize {
			ans += truckSize * p[1]
			break
		}
		truckSize -= p[0]
		ans += p[0] * p[1]
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
		boxTypes := [][]int{{1, 3}, {2, 2}, {3, 1}}
		truckSize := 4
		ans := 8
		assert(maximumUnits(boxTypes, truckSize) == ans)
	}

	{
		boxTypes := [][]int{{5, 10}, {2, 5}, {4, 7}, {3, 9}}
		truckSize := 10
		ans := 91
		assert(maximumUnits(boxTypes, truckSize) == ans)
	}
}
