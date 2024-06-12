// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func accountBalanceAfterPurchase(purchaseAmount int) int {
	return 100 - (purchaseAmount+5)/10*10
}

func main() {
	tests := []struct {
		purchaseAmount int
		ans            int
	}{
		{9, 90},
		{15, 80},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, accountBalanceAfterPurchase(test.purchaseAmount), index)
	}
}
