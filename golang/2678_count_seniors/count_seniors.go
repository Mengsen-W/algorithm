/*
 * @Date: 2023-10-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-23
 * @FilePath: /algorithm/golang/2678_count_seniors/count_seniors.go
 */

// Package main ...
package main

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func countSeniors(details []string) int {
	count := 0
	for i := 0; i < len(details); i++ {
		age, _ := strconv.Atoi(details[i][11:13])
		if age > 60 {
			count++
		}
	}
	return count
}

func main() {
	tests := []struct {
		details []string
		ans     int
	}{
		{[]string{"7868190130M7522", "5303914400F9211", "9273338290F4010"}, 2},
		{[]string{"1313579440F2036", "2921522980M5644"}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countSeniors(test.details), index)
	}
}
