// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func losingPlayer(x, y int) string {
	return [2]string{"Bob", "Alice"}[min(x, y/4)%2]
}

func main() {
	tests := []struct {
		x   int
		y   int
		ans string
	}{
		{2, 7, "Alice"},
		{4, 11, "Bob"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, losingPlayer(test.x, test.y), index)
	}
}
