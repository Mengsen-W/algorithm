/*
 * @Date: 2023-01-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-19
 * @FilePath: /algorithm/golang/2299_strong_password_checker_ii/strong_password_checker_ii.go
 */

package main

import (
	"strings"
	"unicode"
)

func strongPasswordCheckerII(password string) bool {
	n := len(password)
	if n < 8 {
		return false
	}

	var hasLower, hasUpper, hasDigit, hasSpecial bool
	for i, ch := range password {
		if i != n-1 && password[i] == password[i+1] {
			return false
		}
		if unicode.IsLower(ch) {
			hasLower = true
		} else if unicode.IsUpper(ch) {
			hasUpper = true
		} else if unicode.IsDigit(ch) {
			hasDigit = true
		} else if strings.ContainsRune("!@#$%^&*()-+", ch) {
			hasSpecial = true
		}
	}

	return hasLower && hasUpper && hasDigit && hasSpecial
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(strongPasswordCheckerII("IloveLe3tcode!"))
	assert(!strongPasswordCheckerII("Me+You--IsMyDream"))
	assert(!strongPasswordCheckerII("1aB!"))
}
