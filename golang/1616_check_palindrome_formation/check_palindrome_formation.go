/*
 * @Date: 2023-03-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-18
 * @FilePath: /algorithm/golang/1616_check_palindrome_formation/check_palindrome_formation.go
 */

// Package main ...
package main

func checkPalindromeFormation(a, b string) bool {
	checkSelfPalindrome := func(s string) bool {
		left, right := 0, len(s)-1
		for left < right && s[left] == s[right] {
			left++
			right--
		}
		return left >= right
	}

	checkConcatenation := func(a, b string) bool {
		left, right := 0, len(a)-1
		for left < right && a[left] == b[right] {
			left++
			right--
		}
		if left >= right {
			return true
		}
		return checkSelfPalindrome(a[left:right+1]) || checkSelfPalindrome(b[left:right+1])
	}

	return checkConcatenation(a, b) || checkConcatenation(b, a)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		a := "x"
		b := "y"
		ans := true
		assert(checkPalindromeFormation(a, b) == ans)
	}

	{
		a := "abdef"
		b := "fecab"
		ans := true
		assert(checkPalindromeFormation(a, b) == ans)
	}

	{
		a := "ulacfd"
		b := "jizalu"
		ans := true
		assert(checkPalindromeFormation(a, b) == ans)
	}
}
