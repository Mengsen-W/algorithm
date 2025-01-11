// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func generateKey(num1 int, num2 int, num3 int) int {
	key := 0
	for p := 1; num1 > 0 && num2 > 0 && num3 > 0; p *= 10 {
		key += min(num1%10, min(num2%10, num3%10)) * p
		num1, num2, num3 = num1/10, num2/10, num3/10
	}
	return key
}

func main() {
	tests := []struct {
		num1 int
		num2 int
		num3 int
		ans  int
	}{
		{1, 10, 1000, 0},
		{987, 879, 798, 777},
		{1, 2, 3, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, generateKey(test.num1, test.num2, test.num3), index)
	}
}
