/*
 * @Date: 2022-11-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-06
 * @FilePath: /algorithm/1678_interpret/interpret.go
 */

package main

import "strings"

func interpret(command string) string {
	res := &strings.Builder{}
	for i, c := range command {
		if c == 'G' {
			res.WriteByte('G')
		} else if c == '(' {
			if command[i+1] == ')' {
				res.WriteByte('o')
			} else {
				res.WriteString("al")
			}
		}
	}
	return res.String()
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(interpret("G()(al)") == "Goal")
	assert(interpret("G()()()()(al)") == "Gooooal")
	assert(interpret("(al)G(al)()()G") == "alGalooG")
}
