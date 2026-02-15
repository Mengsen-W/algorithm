// Package main ...
package main

import (
	"math/big"
	"testing"

	"github.com/stretchr/testify/assert"
)

func addBinary(a string, b string) string {
	if a == "" {
		return b
	}
	if b == "" {
		return a
	}

	x := new(big.Int)
	x.SetString(a, 2)
	y := new(big.Int)
	y.SetString(b, 2)
	zero := new(big.Int)
	for y.Cmp(zero) != 0 {
		answer := new(big.Int)
		answer.Xor(x, y)

		carry := new(big.Int)
		carry.And(x, y)
		carry.Lsh(carry, 1)

		x.Set(answer)
		y.Set(carry)
	}

	return x.Text(2)
}

func main() {
	tests := []struct {
		a   string
		b   string
		ans string
	}{
		{"11", "1", "100"},
		{"1010", "1011", "10101"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, addBinary(test.a, test.b), index)
	}
}
