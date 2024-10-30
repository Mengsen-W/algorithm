package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func getSmallestString(s string) string {
	r := []rune(s)

	for i := 0; i+1 < len(r); i++ {
		if r[i] > r[i+1] && r[i]%2 == r[i+1]%2 {
			r[i], r[i+1] = r[i+1], r[i]
			break
		}
	}
	return string(r)
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"45320", "43520"},
		{"001", "001"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getSmallestString(test.s), index)
	}
}
