/*
 * @Date: 2022-11-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-05
 * @FilePath: /algorithm/1106_parse_bool_expr/parse_bool_expr.go
 */

package main

func parseBoolExpr(expression string) bool {
	stk := []rune{}
	for _, c := range expression {
		if c == ',' {
			continue
		}
		if c != ')' {
			stk = append(stk, c)
			continue
		}
		t := 0
		f := 0
		for stk[len(stk)-1] != '(' {
			val := stk[len(stk)-1]
			stk = stk[:len(stk)-1]
			if val == 't' {
				t++
			} else {
				f++
			}
		}
		stk = stk[:len(stk)-1]
		op := stk[len(stk)-1]
		stk = stk[:len(stk)-1]
		c = 't'
		switch op {
		case '!':
			if f != 1 {
				c = 'f'
			}
			stk = append(stk, c)
		case '&':
			if f != 0 {
				c = 'f'
			}
			stk = append(stk, c)
		case '|':
			if t == 0 {
				c = 'f'
			}
			stk = append(stk, c)
		}
	}
	return stk[len(stk)-1] == 't'
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(parseBoolExpr("!(f)"))
	assert(parseBoolExpr("|(f,t)"))
	assert(!parseBoolExpr("&(t,f)"))
	assert(!parseBoolExpr("|(&(t,f,t),!(t))"))
}
