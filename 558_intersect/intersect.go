/*
 * @Date: 2022-07-15
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-15
 * @FilePath: /algorithm/558_intersect/intersect.go
 */

package main

import "reflect"

// Node Definition for a QuadTree node.
type Node struct {
	Val         bool
	IsLeaf      bool
	TopLeft     *Node
	TopRight    *Node
	BottomLeft  *Node
	BottomRight *Node
}

func intersect(quadTree1, quadTree2 *Node) *Node {
	if quadTree1.IsLeaf {
		if quadTree1.Val {
			return &Node{Val: true, IsLeaf: true}
		}
		return quadTree2
	}
	if quadTree2.IsLeaf {
		return intersect(quadTree2, quadTree1)
	}
	o1 := intersect(quadTree1.TopLeft, quadTree2.TopLeft)
	o2 := intersect(quadTree1.TopRight, quadTree2.TopRight)
	o3 := intersect(quadTree1.BottomLeft, quadTree2.BottomLeft)
	o4 := intersect(quadTree1.BottomRight, quadTree2.BottomRight)
	if o1.IsLeaf && o2.IsLeaf && o3.IsLeaf && o4.IsLeaf && o1.Val == o2.Val && o1.Val == o3.Val && o1.Val == o4.Val {
		return &Node{Val: o1.Val, IsLeaf: true}
	}
	return &Node{false, false, o1, o2, o3, o4}
}

func main() {
	assert := func(a, b *Node) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		quadTree1 := &Node{false, true, &Node{true, true, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}}
		quadTree2 := &Node{false, true, &Node{true, true, nil, nil, nil, nil}, &Node{false, true, &Node{true, false, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}}, &Node{true, true, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}}
		ans := &Node{false, true, &Node{true, true, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}}
		assert(intersect(quadTree1, quadTree2), ans)
	}

	{
		quadTree1 := &Node{true, false, nil, nil, nil, nil}
		quadTree2 := &Node{true, false, nil, nil, nil, nil}
		ans := &Node{true, false, nil, nil, nil, nil}
		assert(intersect(quadTree1, quadTree2), ans)
	}

	{
		quadTree1 := &Node{false, false, &Node{true, false, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}}
		quadTree2 := &Node{false, false, &Node{true, true, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}}
		ans := &Node{true, true, nil, nil, nil, nil}
		assert(intersect(quadTree1, quadTree2), ans)
	}

	{
		quadTree1 := &Node{false, false, &Node{true, true, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}}
		quadTree2 := &Node{false, false, &Node{true, true, nil, nil, nil, nil}, &Node{false, true, &Node{true, true, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}}, &Node{true, false, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}}
		ans := &Node{false, false, &Node{true, true, nil, nil, nil, nil}, &Node{false, true, &Node{true, true, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}}, &Node{true, true, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}}
		assert(intersect(quadTree1, quadTree2), ans)
	}

	{
		quadTree1 := &Node{false, true, &Node{true, false, &Node{true, false, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}}, &Node{false, true, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}}
		quadTree2 := &Node{false, true, &Node{false, true, &Node{true, false, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}}, &Node{true, false, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}}
		ans := &Node{false, false, &Node{false, true, &Node{true, false, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}}, &Node{false, true, &Node{true, false, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}, &Node{true, true, nil, nil, nil, nil}}, &Node{true, true, nil, nil, nil, nil}, &Node{true, false, nil, nil, nil, nil}}
		assert(intersect(quadTree1, quadTree2), ans)
	}
}
