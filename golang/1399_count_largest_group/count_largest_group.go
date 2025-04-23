// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countLargestGroup(n int) int {
	hashMap := make(map[int]int)
	maxValue := 0
	for i := 1; i <= n; i++ {
		key := 0
		i0 := i
		for i0 > 0 {
			key += i0 % 10
			i0 /= 10
		}
		hashMap[key]++
		maxValue = max(maxValue, hashMap[key])
	}

	count := 0
	for _, value := range hashMap {
		if value == maxValue {
			count++
		}
	}
	return count
}

func main() {
	tests := []struct {
		input  int
		expect int
	}{
		{13, 4},
		{2, 2},
		{15, 6},
		{24, 5},
	}
	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expect, countLargestGroup(test.input), index)
	}
}
