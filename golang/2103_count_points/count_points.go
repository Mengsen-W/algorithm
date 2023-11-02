/*
 * @Date: 2023-11-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-02
 * @FilePath: /algorithm/golang/2103_count_points/count_points.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countPoints(rings string) int {
	const poleNum = 10
	state := make([]int, poleNum)
	n := len(rings)
	for i := 0; i < n; i += 2 {
		color := rings[i]
		poleIndex := rings[i+1] - '0'
		if color == 'R' {
			state[poleIndex] |= 1
		} else if color == 'G' {
			state[poleIndex] |= 2
		} else {
			state[poleIndex] |= 4
		}
	}
	res := 0
	for i := 0; i < poleNum; i++ {
		if state[i] == 7 {
			res++
		}
	}
	return res
}

func main() {
	tests := []struct {
		rings string
		ans   int
	}{
		{"B0B6G0R6R0R6G9", 1},
		{"B0R0G0R9R0B0G0", 1},
		{"G4", 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countPoints(test.rings), index)
	}
}
