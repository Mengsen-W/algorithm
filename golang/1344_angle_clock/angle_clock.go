// Package main ...
package main

import (
	"fmt"
	"math"
)

func angleClock(hour int, minutes int) float64 {
	var oneMinAngle, oneHourAngle, minutesAngle, hourAngle, diff float64
	oneMinAngle = 6
	oneHourAngle = 30

	minutesAngle = oneMinAngle * float64(minutes)
	hourAngle = (float64(hour%12) + float64(minutes)/60.0) * oneHourAngle

	diff = math.Abs(hourAngle - minutesAngle)
	return math.Min(diff, 360-diff)
}

func main() {
	tests := []struct {
		hour    int
		minutes int
		ans     float64
	}{
		{12, 30, 165},
		{3, 30, 75},
		{3, 15, 7.5},
		{4, 50, 155},
		{12, 0, 0},
	}

	for _, test := range tests {
		got := angleClock(test.hour, test.minutes)
		if got != test.ans {
			fmt.Println("test failed", test, got)
		}
	}
}
