/*
 * @Date: 2021-06-17 07:34:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-17 07:44:55
 */

package main

import "fmt"

func isNumber(s string) bool {
	pos := 0
	ls := len(s)
	n := byte(0)
	for pos < ls && s[pos] == ' ' {
		pos++
	}
	// +-
	if pos < ls && (s[pos] == '+' || s[pos] == '-') {
		pos++
	}
	// [0-9]
	if p := scanNumber(s, pos, ls); p > pos {
		n |= 1
		pos = p
	}
	// .
	if pos < ls && s[pos] == '.' {
		pos++
		// [0-9]
		if p := scanNumber(s, pos, ls); p > pos {
			pos = p
			n |= 1 << 1
		}
	}
	// e
	if pos < ls && (s[pos] == 'e' || s[pos] == 'E') {
		pos++
		if pos < ls && (s[pos] == '+' || s[pos] == '-') {
			pos++
		}
		// [0-9]
		if p := scanNumber(s, pos, ls); pos == p {
			return false
		} else {
			pos = p
		}
	}
	for pos < ls && s[pos] == ' ' {
		pos++
	}
	return n > 0 && pos == ls
}

func scanNumber(s string, pos, end int) int {
	for pos < end && s[pos] >= '0' && s[pos] <= '9' {
		pos++
	}
	return pos
}

func main() {
	tests := []struct {
		number string
		ok     bool
	}{
		{"0", true},
		{"+0", true},
		{"-0.2", true},
		{"-+0.2", false},
		{"-0.", false},
		{"-0.2e", false},
		{"-0.2e10", true},
		{"-0.2e-10", true},
		{"-0.2e-1.2", false},
		{"-0.2e-12  ", true},
		{"95a54e53 ", false},
		{".1", true},
		{"1.", true},
		{".", false},
		{".e64", false},
	}
	for _, test := range tests {
		if ok := isNumber(test.number); ok == test.ok {
			fmt.Println("test", test.number, ok, "pass")
		} else {
			fmt.Println("test", test.number, ok, "failed")
		}
	}
}
