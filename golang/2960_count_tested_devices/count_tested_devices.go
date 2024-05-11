/*
 * @Date: 2024-05-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-10
 * @FilePath: /algorithm/golang/2960_count_tested_devices/count_tested_devices.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countTestedDevices(batteryPercentages []int) int {
	need := 0
	for _, battery := range batteryPercentages {
		if battery > need {
			need++
		}
	}
	return need
}

func main() {
	tests := []struct {
		batteryPercentages []int
		ans                int
	}{
		{[]int{1, 1, 2, 1, 3}, 3},
		{[]int{0, 1, 2}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countTestedDevices(test.batteryPercentages), index)
	}
}
