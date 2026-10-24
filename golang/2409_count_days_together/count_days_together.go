// Package main ...
package main

import "strconv"

func countDaysTogether(arriveAlice string, leaveAlice string, arriveBob string, leaveBob string) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	datesOfMonths := []int{31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}
	prefixSum := make([]int, 1)
	for _, date := range datesOfMonths {
		prefixSum = append(prefixSum, prefixSum[len(prefixSum)-1]+date)
	}
	calculateDayOfYear := func(day string, prefixSum []int) int {
		month, _ := strconv.Atoi(day[:2])
		date, _ := strconv.Atoi(day[3:])
		return prefixSum[month-1] + date
	}

	arriveAliceDay := calculateDayOfYear(arriveAlice, prefixSum)
	leaveAliceDay := calculateDayOfYear(leaveAlice, prefixSum)
	arriveBobDay := calculateDayOfYear(arriveBob, prefixSum)
	leaveBobDay := calculateDayOfYear(leaveBob, prefixSum)

	return max(0, min(leaveAliceDay, leaveBobDay)-max(arriveAliceDay, arriveBobDay)+1)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	tests := []struct {
		arriveAlice string
		leaveAlice  string
		arriveBob   string
		leaveBob    string
		ans         int
	}{
		{"08-15", "08-18", "08-16", "08-19", 3},
		{"10-01", "10-31", "11-01", "12-31", 0},
	}

	for _, test := range tests {
		assert(countDaysTogether(test.arriveAlice, test.leaveAlice, test.arriveBob, test.leaveBob) == test.ans)
	}
}
