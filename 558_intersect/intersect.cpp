/*
 * @Date: 2022-07-15
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-15
 * @FilePath: /algorithm/558_intersect/intersect.cpp
 */

#include <cassert>

// Definition for a QuadTree node.
class Node {
 public:
  bool val;
  bool isLeaf;
  Node* topLeft;
  Node* topRight;
  Node* bottomLeft;
  Node* bottomRight;

  Node() {
    val = false;
    isLeaf = false;
    topLeft = nullptr;
    topRight = nullptr;
    bottomLeft = nullptr;
    bottomRight = nullptr;
  }

  Node(bool _val, bool _isLeaf) {
    val = _val;
    isLeaf = _isLeaf;
    topLeft = nullptr;
    topRight = nullptr;
    bottomLeft = nullptr;
    bottomRight = nullptr;
  }

  Node(bool _val, bool _isLeaf, Node* _topLeft, Node* _topRight, Node* _bottomLeft, Node* _bottomRight) {
    val = _val;
    isLeaf = _isLeaf;
    topLeft = _topLeft;
    topRight = _topRight;
    bottomLeft = _bottomLeft;
    bottomRight = _bottomRight;
  }
};

class Solution {
 public:
  Node* intersect(Node* quadTree1, Node* quadTree2) {
    if (quadTree1->isLeaf) {
      if (quadTree1->val) {
        return new Node(true, true);
      }
      return new Node(quadTree2->val, quadTree2->isLeaf, quadTree2->topLeft, quadTree2->topRight, quadTree2->bottomLeft,
                      quadTree2->bottomRight);
    }
    if (quadTree2->isLeaf) {
      return intersect(quadTree2, quadTree1);
    }
    Node* o1 = intersect(quadTree1->topLeft, quadTree2->topLeft);
    Node* o2 = intersect(quadTree1->topRight, quadTree2->topRight);
    Node* o3 = intersect(quadTree1->bottomLeft, quadTree2->bottomLeft);
    Node* o4 = intersect(quadTree1->bottomRight, quadTree2->bottomRight);
    if (o1->isLeaf && o2->isLeaf && o3->isLeaf && o4->isLeaf && o1->val == o2->val && o1->val == o3->val &&
        o1->val == o4->val) {
      return new Node(o1->val, true);
    }
    return new Node(false, false, o1, o2, o3, o4);
  }
};

int main() {
  {
    Node* quadTree1 = new Node(0, 1, new Node(1, 1), new Node(1, 1), new Node(1, 0), new Node(1, 0));
    Node* quadTree2 =
        new Node(0, 1, new Node(1, 1), new Node(0, 1, new Node(1, 0), new Node(1, 0), new Node(1, 1), new Node(1, 1)),
                 new Node(1, 1), new Node(1, 0));
    Node* ans = new Node(0, 1, new Node(1, 1), new Node(1, 1), new Node(1, 1), new Node(1, 0));
    assert(Solution().intersect(quadTree1, quadTree2) == ans);
  }

  {
    Node* quadTree1 = new Node(1, 0);
    Node* quadTree2 = new Node(1, 0);
    Node* ans = new Node(1, 0);
    assert(Solution().intersect(quadTree1, quadTree2) == ans);
  }

  {
    Node* quadTree1 = new Node(0, 0, new Node(1, 0), new Node(1, 0), new Node(1, 1), new Node(1, 1));
    Node* quadTree2 = new Node(0, 0, new Node(1, 1), new Node(1, 1), new Node(1, 0), new Node(1, 1));
    Node* ans = new Node(1, 1);
    assert(Solution().intersect(quadTree1, quadTree2) == ans);
  }

  {
    Node* quadTree1 = new Node(0, 0, new Node(1, 1), new Node(1, 0), new Node(1, 1), new Node(1, 1));
    Node* quadTree2 =
        new Node(0, 0, new Node(1, 1), new Node(0, 1, new Node(1, 1), new Node(1, 0), new Node(1, 0), new Node(1, 1)),
                 new Node(1, 0), new Node(1, 1));
    Node* ans =
        new Node(0, 0, new Node(1, 1), new Node(0, 1, new Node(1, 1), new Node(1, 0), new Node(1, 0), new Node(1, 1)),
                 new Node(1, 1), new Node(1, 1));
    assert(Solution().intersect(quadTree1, quadTree2) == ans);
  }

  {
    Node* quadTree1 = new Node(0, 1, new Node(1, 0, new Node(1, 0), new Node(1, 0), new Node(1, 1), new Node(1, 1)),
                               new Node(0, 1), new Node(1, 1), new Node(1, 0));
    Node* quadTree2 = new Node(0, 1, new Node(0, 1, new Node(1, 0), new Node(1, 0), new Node(1, 1), new Node(1, 1)),
                               new Node(1, 0), new Node(1, 1), new Node(1, 0));
    Node* ans = new Node(0, 0, new Node(0, 1, new Node(1, 0), new Node(1, 0), new Node(1, 1), new Node(1, 1)),
                         new Node(0, 1, new Node(1, 0), new Node(1, 0), new Node(1, 1), new Node(1, 1)), new Node(1, 1),
                         new Node(1, 0));
    assert(Solution().intersect(quadTree1, quadTree2) == ans);
  }
}