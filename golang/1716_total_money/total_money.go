// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func totalMoney(n int) (ans int) {
	// 所有完整的周存的钱
	weekNum := n / 7
	firstWeekMoney := (1 + 7) * 7 / 2
	lastWeekMoney := firstWeekMoney + 7*(weekNum-1)
	weekMoney := (firstWeekMoney + lastWeekMoney) * weekNum / 2
	// 剩下的不能构成一个完整的周的天数里存的钱
	dayNum := n % 7
	firstDayMoney := 1 + weekNum
	lastDayMoney := firstDayMoney + dayNum - 1
	dayMoney := (firstDayMoney + lastDayMoney) * dayNum / 2
	return weekMoney + dayMoney
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{4, 10},
		{10, 37},
		{20, 96},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, totalMoney(test.n), index)
	}
}
