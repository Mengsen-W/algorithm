// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximum69Number(num int) int {
	digitBase := int(math.Pow10(int(math.Log10(float64(num)))))
	for digitBase > 0 {
		if (num/digitBase)%10 == 6 {
			num += 3 * digitBase
			return num
		}
		digitBase /= 10
	}

	return num
}

func main() {
	tests := []struct {
		num    int
		expect int
	}{
		{9669, 9969},
		{9996, 9999},
		{9999, 9999},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expect, maximum69Number(test.num), index)
	}
}
