/*
 * @Date: 2023-12-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-08
 * @FilePath: /algorithm/golang/2008_max_taxi_earnings/max_taxi_earnings.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxTaxiEarnings(n int, rides [][]int) int64 {
	dp := make([]int64, n+1)
	rideMap := map[int][][]int{}
	for _, ride := range rides {
		rideMap[ride[1]] = append(rideMap[ride[1]], ride)
	}
	for i := 1; i <= n; i++ {
		dp[i] = dp[i-1]
		for _, ride := range rideMap[i] {
			dp[i] = max(dp[i], dp[ride[0]]+int64(ride[1]-ride[0]+ride[2]))
		}
	}
	return dp[n]
}

func main() {
	tests := []struct {
		n     int
		rides [][]int
		ans   int64
	}{
		{5, [][]int{{2, 5, 4}, {1, 5, 1}}, 7},
		{20, [][]int{{1, 6, 1}, {3, 10, 2}, {10, 12, 3}, {11, 12, 2}, {12, 15, 2}, {13, 18, 1}}, 20},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxTaxiEarnings(test.n, test.rides), index)
	}
}
