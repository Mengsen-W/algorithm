// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func squareIsWhite(coordinates string) bool {
	return ((coordinates[0]-'a'+1)+(coordinates[1]-'0'))%2 == 1
}

func main() {
	tests := []struct {
		coordinates string
		ans         bool
	}{
		{"a1", false},
		{"h3", true},
		{"c7", false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, squareIsWhite(test.coordinates), index)
	}
}
