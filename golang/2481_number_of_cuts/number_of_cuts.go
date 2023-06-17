/*
 * @Date: 2023-06-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-17
 * @FilePath: /algorithm/golang/2481_number_of_cuts/number_of_cuts.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/go-playground/assert/v2"
)

func numberOfCuts(n int) int {
	if n == 1 {
		return 0
	}
	if n%2 == 0 {
		return n / 2
	}
	return n
}

func main() {
	assert.Equal(&testing.B{}, numberOfCuts(4), 2)
	assert.Equal(&testing.B{}, numberOfCuts(3), 3)
}
