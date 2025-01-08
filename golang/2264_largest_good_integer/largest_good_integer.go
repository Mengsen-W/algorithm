// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func largestGoodInteger(num string) string {
	n := len(num)
	var res string
	for i := 0; i < n-2; i++ {
		if num[i] == num[i+1] && num[i+1] == num[i+2] {
			res = max(res, num[i:i+3])
		}
	}
	return res
}

func main() {
	tests := []struct {
		num string
		ans string
	}{
		{"6777133339", "777"},
		{"2300019", "000"},
		{"42352338", ""},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, largestGoodInteger(test.num), index)
	}
}
