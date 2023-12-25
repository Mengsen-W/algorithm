/*
 * @Date: 2023-12-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-25
 * @FilePath: /algorithm/golang/1276_num_of_burgers/num_of_burgers.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numOfBurgers(tomatoSlices int, cheeseSlices int) []int {
	if tomatoSlices%2 != 0 || tomatoSlices < cheeseSlices*2 || cheeseSlices*4 < tomatoSlices {
		return nil
	}
	return []int{tomatoSlices/2 - cheeseSlices, cheeseSlices*2 - tomatoSlices/2}
}

func main() {
	tests := []struct {
		tomatoSlices int
		cheeseSlices int
		ans          []int
	}{
		{16, 7, []int{1, 6}},
		{17, 4, nil},
		{4, 17, nil},
		{0, 0, []int{0, 0}},
		{2, 1, []int{0, 1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numOfBurgers(test.tomatoSlices, test.cheeseSlices), index)
	}
}
