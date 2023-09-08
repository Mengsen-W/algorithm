/*
 * @Date: 2023-09-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-08
 * @FilePath: /algorithm/golang/2651_find_delayed_arrival_time/find_delayed_arrival_time.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findDelayedArrivalTime(arrivalTime int, delayedTime int) int {
	return (arrivalTime + delayedTime) % 24
}

func main() {
	tests := []struct {
		arrivalTime int
		delayTime   int
		ans         int
	}{
		{15, 5, 20},
		{13, 11, 0},
	}

	for i, v := range tests {
		assert.Equal(&testing.T{}, v.ans, findDelayedArrivalTime(v.arrivalTime, v.delayTime), i)
	}
}
