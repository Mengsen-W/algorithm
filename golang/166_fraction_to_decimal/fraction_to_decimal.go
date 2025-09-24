// Package main ...
package main

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func fractionToDecimal(numerator, denominator int) string {
	if numerator%denominator == 0 {
		return strconv.Itoa(numerator / denominator)
	}

	s := []byte{}
	if numerator < 0 != (denominator < 0) {
		s = append(s, '-')
	}

	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	// 整数部分
	numerator = abs(numerator)
	denominator = abs(denominator)
	integerPart := numerator / denominator
	s = append(s, strconv.Itoa(integerPart)...)
	s = append(s, '.')

	// 小数部分
	indexMap := map[int]int{}
	remainder := numerator % denominator
	for remainder != 0 && indexMap[remainder] == 0 {
		indexMap[remainder] = len(s)
		remainder *= 10
		s = append(s, '0'+byte(remainder/denominator))
		remainder %= denominator
	}
	if remainder > 0 { // 有循环节
		insertIndex := indexMap[remainder]
		s = append(s[:insertIndex], append([]byte{'('}, s[insertIndex:]...)...)
		s = append(s, ')')
	}

	return string(s)
}

func main() {
	tests := []struct {
		numerator, denominator int
		expected               string
	}{
		{1, 2, "0.5"},
		{2, 1, "2"},
		{4, 333, "0.(012)"},
		{1, 5, "0.2"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expected, fractionToDecimal(test.numerator, test.denominator), index)
	}
}
