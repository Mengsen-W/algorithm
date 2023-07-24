/*
 * @Date: 2023-07-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-24
 * @FilePath: /algorithm/golang/771_num_jewels_in_stones/num_jewels_in_stones.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numJewelsInStones(jewels string, stones string) int {
	jewelsCount := 0
	jewelsSet := map[byte]bool{}
	for i := range jewels {
		jewelsSet[jewels[i]] = true
	}
	for i := range stones {
		if jewelsSet[stones[i]] {
			jewelsCount++
		}
	}
	return jewelsCount
}

func main() {
	tests := []struct {
		jewels string
		stones string
		ans    int
	}{
		{"aA", "aAAbbbb", 3},
		{"z", "ZZ", 0},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, numJewelsInStones(item.jewels, item.stones))
	}
}
