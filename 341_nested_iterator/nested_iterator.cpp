/*
 * @Date: 2021-03-23 08:50:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-23 08:52:02
 */

// This is the interface that allows for creating nested lists.
// You should not implement it, or speculate about its implementation
class NestedInteger {
 public:
  // Return true if this NestedInteger holds a single integer, rather than
  a nested list.bool isInteger() const;

  // Return the single integer that this NestedInteger holds, if it holds a
  // single integer
  // The result is undefined if this NestedInteger holds a nested list
  int getInteger() const;

  // Return the nested list that this NestedInteger holds, if it holds a
  // nested list
  // The result is undefined if this NestedInteger holds a single integer
  const vector<NestedInteger> &getList() const;
};

class NestedIterator {
 private:
  stack<NestedInteger> st;

 public:
  NestedIterator(vector<NestedInteger> &nestedList) {
    for (auto iter = nestedList.rbegin(); iter != nestedList.rend(); iter++) {
      st.push(*iter);
    }
  }

  int next() {
    auto t = st.top();
    st.pop();
    return t.getInteger();
  }

  bool hasNext() {
    while (!st.empty()) {
      auto cur = st.top();
      if (cur.isInteger()) return true;
      st.pop();
      auto curList = cur.getList();
      for (auto iter = curList.rbegin(); iter != curList.rend(); iter++) {
        st.push(*iter);
      }
    }
    return false;
  }
};

/**
 * Your NestedIterator object will be instantiated and called as such:
 * NestedIterator i(nestedList);
 * while (i.hasNext()) cout << i.next();
 */