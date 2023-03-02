/*
 * @Date: 2023-03-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-02
 * @FilePath: /algorithm/golang/05.02_print_bin/print_bin.go
 */

package main

import "strings"

func printBin(num float64) string {
	sb := &strings.Builder{}
	sb.WriteString("0.")
	for sb.Len() <= 32 && num != 0 {
		num *= 2
		digit := byte(num)
		sb.WriteByte('0' + digit)
		num -= float64(digit)
	}
	if sb.Len() <= 32 {
		return sb.String()
	}
	return "ERROR"
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		num := 0.625
		ans := "0.101"
		assert(printBin(num) == ans)
	}

	{
		num := 0.1
		ans := "ERROR"
		assert(printBin(num) == ans)
	}
}
