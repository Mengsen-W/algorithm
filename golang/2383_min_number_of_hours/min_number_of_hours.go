/*
 * @Date: 2023-03-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-13
 * @FilePath: /algorithm/golang/2383_min_number_of_hours/min_number_of_hours.go
 */

// Package main ...
package main

func minNumberOfHours(initialEnergy int, initialExperience int, energy []int, experience []int) int {
	sum := 0
	for _, e := range energy {
		sum += e
	}
	trainingHours := 0
	if initialEnergy <= sum {
		trainingHours = sum + 1 - initialEnergy
	}
	for _, e := range experience {
		if initialExperience <= e {
			trainingHours += 1 + (e - initialExperience)
			initialExperience = 2*e + 1
		} else {
			initialExperience += e
		}
	}
	return trainingHours
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		initialEnergy := 5
		initialExperience := 3
		energy := []int{1, 4, 3, 2}
		experience := []int{2, 6, 3, 1}
		ans := 8
		assert(minNumberOfHours(initialEnergy, initialExperience, energy, experience) == ans)
	}

	{
		initialEnergy := 2
		initialExperience := 4
		energy := []int{1}
		experience := []int{3}
		ans := 0
		assert(minNumberOfHours(initialEnergy, initialExperience, energy, experience) == ans)
	}
}
