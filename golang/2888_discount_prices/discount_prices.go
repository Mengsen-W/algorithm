// Package main ...
package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func discountPrices(sentence string, discount int) string {
	words := strings.Split(sentence, " ")
	for i, word := range words {
		if strings.HasPrefix(word, "$") && isNumeric(word[1:]) {
			price, _ := strconv.Atoi(word[1:])
			discountedPrice := float64(price) * (1 - float64(discount)/100)
			words[i] = fmt.Sprintf("$%.2f", discountedPrice)
		}
	}
	return strings.Join(words, " ")
}

func isNumeric(s string) bool {
	match, _ := regexp.MatchString("^[0-9]+$", s)
	return match
}

func main() {
	tests := []struct {
		sentence string
		discount int
		ans      string
	}{
		{"there are $1 $2 and 5$ candies in the shop", 50, "there are $0.50 $1.00 and 5$ candies in the shop"},
		{"1 2 $3 4 $5 $6 7 8$ $9 $10$", 100, "1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, discountPrices(test.sentence, test.discount), index)
	}
}
