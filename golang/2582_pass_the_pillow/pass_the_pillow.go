/*
 * @Date: 2023-09-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-26
 * @FilePath: /algorithm/golang/2582_pass_the_pillow/pass_the_pillow.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func passThePillow(n int, time int) int {
	time %= (n - 1) * 2
	if time < n {
		return time + 1
	}
	return n*2 - time - 1
}

func main() {
	tests := []struct {
		n    int
		time int
		ans  int
	}{
		{4, 5, 2},
		{3, 2, 3},
	}

	for index, item := range tests {
		assert.Equal(&testing.T{}, item.ans, passThePillow(item.n, item.time), index)
	}
}
