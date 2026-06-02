// Package main ...
package main

func earliestFinishTime(landStartTime []int, landDuration []int, waterStartTime []int, waterDuration []int) int {
	n := len(landStartTime)
	m := len(waterStartTime)
	res := 1000000
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			land := landStartTime[i] + landDuration[i]
			landWater := max(waterStartTime[j], land)
			landWater += waterDuration[j]
			if landWater < res {
				res = landWater
			}

			water := waterStartTime[j] + waterDuration[j]
			waterLand := max(landStartTime[i], water)
			waterLand += landDuration[i]
			if waterLand < res {
				res = waterLand
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		landStartTime  []int
		landDuration   []int
		waterStartTime []int
		waterDuration  []int
		ans            int
	}{
		{[]int{2, 8}, []int{4, 1}, []int{6}, []int{3}, 9},
		{[]int{5}, []int{3}, []int{1}, []int{10}, 14},
	}

	for _, test := range tests {
		if earliestFinishTime(test.landStartTime, test.landDuration, test.waterStartTime, test.waterDuration) != test.ans {
			panic("test failed")
		}
	}
}
