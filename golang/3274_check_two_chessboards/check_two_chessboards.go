// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func checkTwoChessboards(coordinate1 string, coordinate2 string) bool {
	return (int(coordinate1[0])-int(coordinate2[0])+int(coordinate1[1])-int(coordinate2[1]))%2 == 0
}

func main() {
	tests := []struct {
		coordinate1 string
		coordinate2 string
		ans         bool
	}{
		{"a1", "c3", true},
		{"a1", "h3", false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, checkTwoChessboards(test.coordinate1, test.coordinate2), index)
	}
}
