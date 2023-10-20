/*
 * @Date: 2023-10-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-20
 * @FilePath: /algorithm/golang/2525_categorize_box/categorize_box.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func categorizeBox(length, width, height, mass int) string {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	maxd := max(length, max(width, height))
	vol := length * width * height
	isBulky := maxd >= 10000 || vol >= 1e9
	isHeavy := mass >= 100
	if isBulky && isHeavy {
		return "Both"
	} else if isBulky {
		return "Bulky"
	} else if isHeavy {
		return "Heavy"
	} else {
		return "Neither"
	}
}

func main() {
	tests := []struct {
		length int
		width  int
		height int
		mass   int
		ans    string
	}{
		{1000, 35, 700, 300, "Heavy"},
		{200, 50, 800, 50, "Neither"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, categorizeBox(test.length, test.width, test.height, test.mass), index)
	}
}
