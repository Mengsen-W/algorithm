/*
 * @Date: 2023-09-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-23
 * @FilePath: /algorithm/cpp/1993_LockingTree/LockingTree.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class LockingTree {
 public:
  LockingTree(vector<int>&& parent) {
    int n = parent.size();
    this->parent = parent;
    this->lockNodeUser = vector<int>(n, -1);
    this->children = vector<vector<int>>(n);
    for (int i = 0; i < n; i++) {
      int p = parent[i];
      if (p != -1) {
        children[p].emplace_back(i);
      }
    }
  }

  bool lock(int num, int user) {
    if (lockNodeUser[num] == -1) {
      lockNodeUser[num] = user;
      return true;
    }
    return false;
  }

  bool unlock(int num, int user) {
    if (lockNodeUser[num] == user) {
      lockNodeUser[num] = -1;
      return true;
    }
    return false;
  }

  bool upgrade(int num, int user) {
    bool res = lockNodeUser[num] == -1 && !hasLockedAncestor(num) && checkAndUnlockDescendant(num);
    if (res) {
      lockNodeUser[num] = user;
    }
    return res;
  }

  bool hasLockedAncestor(int num) {
    num = parent[num];
    while (num != -1) {
      if (lockNodeUser[num] != -1) {
        return true;
      }
      num = parent[num];
    }
    return false;
  }

  bool checkAndUnlockDescendant(int num) {
    bool res = lockNodeUser[num] != -1;
    lockNodeUser[num] = -1;
    for (int child : children[num]) {
      res |= checkAndUnlockDescendant(child);
    }
    return res;
  }

 private:
  vector<int> parent;
  vector<int> lockNodeUser;
  vector<vector<int>> children;
};

int main() {
  LockingTree* lockingTree = new LockingTree({-1, 0, 0, 1, 1, 2, 2});
  assert(lockingTree->lock(2, 2));
  assert(!lockingTree->unlock(2, 3));
  assert(lockingTree->unlock(2, 2));
  assert(lockingTree->lock(4, 5));
  assert(lockingTree->upgrade(0, 1));
  assert(!lockingTree->lock(0, 1));
}