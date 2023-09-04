/*
 * @Date: 2023-09-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-04
 * @FilePath: /algorithm/cpp/449_Codec/codec.cpp
 */

//  Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

#include <cassert>
#include <stack>
#include <string>
#include <vector>

using namespace std;

class Codec {
 public:
  string serialize(TreeNode *root) {
    string res;
    vector<int> arr;
    postOrder(root, arr);
    if (arr.size() == 0) {
      return res;
    }
    for (int i = 0; i < arr.size() - 1; i++) {
      res.append(to_string(arr[i]) + ",");
    }
    res.append(to_string(arr.back()));
    return res;
  }

  vector<string> split(const string &str, char dec) {
    int pos = 0;
    int start = 0;
    vector<string> res;
    while (pos < str.size()) {
      while (pos < str.size() && str[pos] == dec) {
        pos++;
      }
      start = pos;
      while (pos < str.size() && str[pos] != dec) {
        pos++;
      }
      if (start < str.size()) {
        res.emplace_back(str.substr(start, pos - start));
      }
    }
    return res;
  }

  TreeNode *deserialize(string data) {
    if (data.size() == 0) {
      return nullptr;
    }
    vector<string> arr = split(data, ',');
    stack<int> st;
    for (auto &str : arr) {
      st.emplace(stoi(str));
    }
    return construct(INT_MIN, INT_MAX, st);
  }

  void postOrder(TreeNode *root, vector<int> &arr) {
    if (root == nullptr) {
      return;
    }
    postOrder(root->left, arr);
    postOrder(root->right, arr);
    arr.emplace_back(root->val);
  }

  TreeNode *construct(int lower, int upper, stack<int> &st) {
    if (st.size() == 0 || st.top() < lower || st.top() > upper) {
      return nullptr;
    }
    int val = st.top();
    st.pop();
    TreeNode *root = new TreeNode(val);
    root->right = construct(val, upper, st);
    root->left = construct(lower, val, st);
    return root;
  }
};

int main() {
  vector<string> tests{"2,1,3", ""};
  Codec *codec = new Codec();
  for (auto test : tests) {
    assert(codec->serialize(codec->deserialize(test)) == test);
  }
}