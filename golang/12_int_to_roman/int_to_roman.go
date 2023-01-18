/*
 * @Date: 2021-05-14 08:44:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-14 08:49:37
 */

package main

var (
	thousands = []string{"", "M", "MM", "MMM"}
	hundreds  = []string{"", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"}
	tens      = []string{"", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"}
	ones      = []string{"", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"}
)

func intToRoman(num int) string {
	return thousands[num/1000] + hundreds[num%1000/100] + tens[num%100/10] + ones[num%10]
}

func main() {
	if intToRoman(3) != "III" {
		panic("Not Passed!")
	}
	if intToRoman(4) != "IV" {
		panic("Not Passed!")
	}
	if intToRoman(9) != "IX" {
		panic("Not Passed!")
	}
	if intToRoman(58) != "LVIII" {
		panic("Not Passed!")
	}
	if intToRoman(1994) != "MCMXCIV" {
		panic("Not Passed!")
	}

}
