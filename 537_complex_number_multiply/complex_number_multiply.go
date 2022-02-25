/*
 * @Date: 2022-02-25 00:25:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-25 00:34:26
 * @FilePath: /algorithm/537_complex_number_multiply/complex_number_multiply.go
 */

package main

import (
	"fmt"
	"strconv"
	"strings"
)

func parseComplexNumber(num string) (real, imag int) {
	i := strings.IndexByte(num, '+')
	real, _ = strconv.Atoi(num[:i])
	imag, _ = strconv.Atoi(num[i+1 : len(num)-1])
	return
}

func complexNumberMultiply(num1, num2 string) string {
	real1, imag1 := parseComplexNumber(num1)
	real2, imag2 := parseComplexNumber(num2)
	return fmt.Sprintf("%d+%di", real1*real2-imag1*imag2, real1*imag2+imag1*real2)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(complexNumberMultiply("1+1i", "1+1i") == "0+2i")
	assert(complexNumberMultiply("1+-1i", "1+-1i") == "0+-2i")
}
