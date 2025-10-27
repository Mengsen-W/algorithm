// Package main ...
package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfBeams(bank []string) int {
	last, ans := 0, 0
	for _, line := range bank {
		cnt := strings.Count(line, "1")
		if cnt != 0 {

			ans += last * cnt
			last = cnt
		}
	}
	return ans
}

func main() {
	tests := []struct {
		bank []string
		ans  int
	}{
		{[]string{"011001", "000000", "010100", "001000"}, 8},
		{[]string{"000", "111", "000"}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfBeams(test.bank), index)
	}
}
