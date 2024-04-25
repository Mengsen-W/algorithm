/*
 * @Date: 2024-04-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-25
 * @FilePath: /algorithm/golang/2739_distance_traveled/distance_traveled.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func distanceTraveled(mainTank int, additionalTank int) int {
	ans := 0
	for mainTank >= 5 {
		mainTank -= 5
		ans += 50
		if additionalTank > 0 {
			additionalTank--
			mainTank++
		}
	}
	return ans + mainTank*10
}

func main() {
	tests := []struct {
		mainTank       int
		additionalTank int
		ans            int
	}{
		{5, 10, 60},
		{1, 2, 10},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, distanceTraveled(test.mainTank, test.additionalTank), index)
	}
}
