/*
 * @Date: 2021-10-13 08:45:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-13 08:53:48
 */

package main

import (
	"reflect"
	"strconv"
	"strings"
)

func fizzBuzz(n int) (ans []string) {
	for i := 1; i <= n; i++ {
		sb := &strings.Builder{}
		if i%3 == 0 {
			sb.WriteString("Fizz")
		}
		if i%5 == 0 {
			sb.WriteString("Buzz")
		}
		if sb.Len() == 0 {
			sb.WriteString(strconv.Itoa(i))
		}
		ans = append(ans, sb.String())
	}
	return
}

func main() {
	if !reflect.DeepEqual(fizzBuzz(15),
		[]string{"1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8",
			"Fizz", "Buzz", "11", "Fizz", "13", "14",
			"FizzBuzz"}) {
		panic("Not Passed")
	}
}
