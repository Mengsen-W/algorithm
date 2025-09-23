// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func compareVersion(version1, version2 string) int {
	n, m := len(version1), len(version2)
	i, j := 0, 0
	for i < n || j < m {
		x := 0
		for ; i < n && version1[i] != '.'; i++ {
			x = x*10 + int(version1[i]-'0')
		}
		i++ // 跳过点号
		y := 0
		for ; j < m && version2[j] != '.'; j++ {
			y = y*10 + int(version2[j]-'0')
		}
		j++ // 跳过点号
		if x > y {
			return 1
		}
		if x < y {
			return -1
		}
	}
	return 0
}

func main() {
	tests := []struct {
		version1 string
		version2 string
		ans      int
	}{
		{
			"1.01",
			"1.001",
			0,
		},
		{
			"1.0",
			"1.0.0",
			0,
		},
		{
			"0.1",
			"1.1",
			-1,
		},
		{
			"1.0.1",
			"1",
			1,
		},
		{
			"7.5.2.4",
			"7.5.3",
			-1,
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, compareVersion(test.version1, test.version2), index)
	}
}
