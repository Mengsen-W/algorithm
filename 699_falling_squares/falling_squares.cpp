/*
 * @Date: 2022-05-26 10:19:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-26 10:51:06
 * @FilePath: /algorithm/699_falling_squares/falling_squares.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
  struct Node {
    int l, r, h, R;
    Node *left, *right;
  };

  Node* insert(Node* root, int l, int r, int h) {
    if (!root) return new Node{l, r, h, r};
    auto& p = l > root->l ? root->right : root->left;
    p = insert(p, l, r, h);
    root->R = max(r, root->R);
    return root;
  }

  int search(Node* root, int l, int r) {
    if (!root || l >= root->R) return 0;
    int h = max(l < root->r && r > root->l ? root->h : 0, search(root->left, l, r));
    if (r > root->l) h = max(h, search(root->right, l, r));
    return h;
  }

 public:
  vector<int> fallingSquares(vector<vector<int>> positions) {
    vector ans(positions.size(), 0);
    transform(positions.cbegin(), positions.cend(), ans.begin(), [&, root = (Node*)nullptr, tmp = 0](auto&& p) mutable {
      int h = search(root, p[0], p[0] + p[1]);
      root = insert(root, p[0], p[0] + p[1], h + p[1]);
      return tmp = max(tmp, h + p[1]);
    });
    return ans;
  }
};

int main() {
  assert((Solution().fallingSquares(vector<vector<int>>{{1, 2}, {2, 3}, {6, 1}}) == vector<int>{2, 5, 5}));
  assert((Solution().fallingSquares(vector<vector<int>>{{100, 100}, {200, 100}}) == vector<int>{100, 100}));
}