/*
 * @Date: 2024-03-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-11
 * @FilePath: /algorithm/golang/2129_capitalize_title/capitalize_title.go
 */

// Package main ...
package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func capitalizeTitle(title string) string {
	a := strings.Split(title, " ")
	for i, s := range a {
		s = strings.ToLower(s)
		if len(s) > 2 {
			s = strings.Title(s)
		}
		a[i] = s
	}
	return strings.Join(a, " ")
}

func main() {
	tests := []struct {
		title string
		ans   string
	}{
		{"capiTalIze tHe titLe", "Capitalize The Title"},
		{"First leTTeR of EACH Word", "First Letter of Each Word"},
		{"i lOve leetcode", "i Love Leetcode"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, capitalizeTitle(test.title), index)
	}
}
