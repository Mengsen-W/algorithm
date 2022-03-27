/*
 * @Date: 2022-03-27 02:45:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-27 02:59:48
 * @FilePath: /algorithm/2028_missing_rolls/missing_rolls.go
 */

package main

import "reflect"

func missingRolls(rolls []int, mean, n int) []int {
	missingSum := mean * (n + len(rolls))
	for _, roll := range rolls {
		missingSum -= roll
	}
	if missingSum < n || missingSum > n*6 {
		return nil
	}

	quotient, remainder := missingSum/n, missingSum%n
	ans := make([]int, n)
	for i := range ans {
		ans[i] = quotient
		if i < remainder {
			ans[i]++
		}
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		rolls := []int{3, 2, 4, 3}
		mean := 4
		n := 2
		ans := []int{6, 6}
		assert(missingRolls(rolls, mean, n), ans)
	}

	{
		rolls := []int{1, 5, 6}
		mean := 3
		n := 4
		ans := []int{3, 2, 2, 2}
		assert(missingRolls(rolls, mean, n), ans)
	}

	{
		rolls := []int{1, 2, 3, 4}
		mean := 5
		n := 4
		var ans []int = nil
		assert(missingRolls(rolls, mean, n), ans)
	}

	{
		rolls := []int{1}
		mean := 3
		n := 1
		ans := []int{5}
		assert(missingRolls(rolls, mean, n), ans)
	}
}
