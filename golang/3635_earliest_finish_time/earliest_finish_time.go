// Package main ...
package main

func earliestFinishTime(landStartTime []int, landDuration []int, waterStartTime []int, waterDuration []int) int {
	solve := func(start1, duration1, start2, duration2 []int) int {
		finish1 := 2147483647
		for i := 0; i < len(start1); i++ {
			if val := start1[i] + duration1[i]; val < finish1 {
				finish1 = val
			}
		}
		finish2 := 2147483647
		for i := 0; i < len(start2); i++ {
			curStart := max(finish1, start2[i])
			if val := curStart + duration2[i]; val < finish2 {
				finish2 = val
			}
		}
		return finish2
	}

	landWater := solve(landStartTime, landDuration, waterStartTime, waterDuration)
	waterLand := solve(waterStartTime, waterDuration, landStartTime, landDuration)
	if landWater < waterLand {
		return landWater
	}
	return waterLand
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
		if res := earliestFinishTime(test.landStartTime, test.landDuration, test.waterStartTime, test.waterDuration); res != test.ans {
			println("got", res, "expected", test.ans)
		}
	}
}
