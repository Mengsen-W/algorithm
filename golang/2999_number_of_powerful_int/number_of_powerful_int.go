// Package main ...
package main

import (
	"math"
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfPowerfulInt(start int64, finish int64, limit int, s string) int64 {
	newStart := strconv.FormatInt(start-1, 10)
	newFinish := strconv.FormatInt(finish, 10)
	return calculate(newFinish, s, limit) - calculate(newStart, s, limit)
}

func calculate(x string, s string, limit int) int64 {
	if len(x) < len(s) {
		return 0
	}
	if len(x) == len(s) {
		if x >= s {
			return 1
		}
		return 0
	}

	suffix := x[len(x)-len(s):]
	var count int64
	preLen := len(x) - len(s)

	for i := 0; i < preLen; i++ {
		digit := int(x[i] - '0')
		if limit < digit {
			count += int64(math.Pow(float64(limit+1), float64(preLen-i)))
			return count
		}
		count += int64(digit) * int64(math.Pow(float64(limit+1), float64(preLen-1-i)))
	}
	if suffix >= s {
		count++
	}
	return count
}

func main() {
	tests := []struct {
		start  int64
		finish int64
		limit  int
		s      string
		ans    int64
	}{
		{1, 6000, 4, "124", 5},
		{15, 215, 6, "10", 2},
		{1000, 2000, 4, "3000", 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfPowerfulInt(test.start, test.finish, test.limit, test.s), index)
	}
}
