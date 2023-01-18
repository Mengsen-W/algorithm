/*
 * @Date: 2022-05-02 07:41:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-02 07:45:57
 * @FilePath: /algorithm/591_tag_is_valid/is_valid.go
 */

package main

import (
	"strings"
	"unicode"
)

func isValid(code string) bool {
	tags := []string{}
	for code != "" {
		if code[0] != '<' {
			if len(tags) == 0 {
				return false
			}
			code = code[1:]
			continue
		}
		if len(code) == 1 {
			return false
		}
		if code[1] == '/' {
			j := strings.IndexByte(code, '>')
			if j == -1 {
				return false
			}
			if len(tags) == 0 || tags[len(tags)-1] != code[2:j] {
				return false
			}
			tags = tags[:len(tags)-1]
			code = code[j+1:]
			if len(tags) == 0 && code != "" {
				return false
			}
		} else if code[1] == '!' {
			if len(tags) == 0 || len(code) < 9 || code[2:9] != "[CDATA[" {
				return false
			}
			j := strings.Index(code, "]]>")
			if j == -1 {
				return false
			}
			code = code[j+1:]
		} else {
			j := strings.IndexByte(code, '>')
			if j == -1 {
				return false
			}
			tagName := code[1:j]
			if tagName == "" || len(tagName) > 9 {
				return false
			}
			for _, ch := range tagName {
				if !unicode.IsUpper(ch) {
					return false
				}
			}
			tags = append(tags, tagName)
			code = code[j+1:]
		}
	}
	return len(tags) == 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(isValid("<DIV>This is the first line <![CDATA[<div>]]></DIV>"))
	assert(isValid("<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>"))
	assert(!isValid("<A>  <B> </A>   </B>"))
}
