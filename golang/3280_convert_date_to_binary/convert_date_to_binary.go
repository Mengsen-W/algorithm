// Package main ...
package main

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func binary(x int) string {
	var s []byte
	for ; x != 0; x >>= 1 {
		s = append(s, '0'+byte(x&1))
	}
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		s[i], s[j] = s[j], s[i]
	}
	return string(s)
}

func convertDateToBinary(date string) string {
	year, _ := strconv.Atoi(date[:4])
	month, _ := strconv.Atoi(date[5:7])
	day, _ := strconv.Atoi(date[8:])
	return binary(year) + "-" + binary(month) + "-" + binary(day)
}

func main() {
	tests := []struct {
		date string
		ans  string
	}{
		{"2080-02-29", "100000100000-10-11101"},
		{"1900-01-01", "11101101100-1-1"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, convertDateToBinary(test.date), index)
	}
}
