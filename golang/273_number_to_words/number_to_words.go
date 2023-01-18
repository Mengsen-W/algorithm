/*
 * @Date: 2021-10-11 08:51:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-11 09:18:50
 */

package main

import "strings"

var (
	singles   = []string{"", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"}
	teens     = []string{"Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"}
	tens      = []string{"", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"}
	thousands = []string{"", "Thousand", "Million", "Billion"}
)

func numberToWords(num int) string {
	if num == 0 {
		return "Zero"
	}
	sb := &strings.Builder{}
	toEnglish := func(num int) {
		if num >= 100 {
			sb.WriteString(singles[num/100])
			sb.WriteString(" Hundred ")
			num %= 100
		}
		if num >= 20 {
			sb.WriteString(tens[num/10])
			sb.WriteByte(' ')
			num %= 10
		}
		if 0 < num && num < 10 {
			sb.WriteString(singles[num])
			sb.WriteByte(' ')
		} else if num >= 10 {
			sb.WriteString(teens[num-10])
			sb.WriteByte(' ')
		}
	}
	for i, unit := 3, int(1e9); i >= 0; i-- {
		if curNum := num / unit; curNum > 0 {
			num -= curNum * unit
			toEnglish(curNum)
			sb.WriteString(thousands[i])
			sb.WriteByte(' ')
		}
		unit /= 1000
	}
	return strings.TrimSpace(sb.String())
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(numberToWords(123) == "One Hundred Twenty Three")
	assert(numberToWords(12345) == "Twelve Thousand Three Hundred Forty Five")
	assert(numberToWords(1234567) == "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven")
	assert(numberToWords(1231234567891) == "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One")
}
