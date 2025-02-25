#include <cassert>
#include <vector>

using namespace std;

class Allocator {
 public:
  Allocator(int n) : n(n), memory(n) {}

  int allocate(int size, int mID) {
    int count = 0;
    for (int i = 0; i < n; ++i) {
      if (memory[i]) {
        count = 0;
      } else {
        ++count;
        if (count == size) {
          for (int j = i - count + 1; j <= i; ++j) {
            memory[j] = mID;
          }
          return i - count + 1;
        }
      }
    }
    return -1;
  }

  int freeMemory(int mID) {
    int count = 0;
    for (int i = 0; i < n; ++i) {
      if (memory[i] == mID) {
        ++count;
        memory[i] = 0;
      }
    }
    return count;
  }

 private:
  int n;
  vector<int> memory;
};

int main() {
  Allocator *loc = new Allocator(10);
  assert(loc->allocate(1, 1) == 0);
  assert(loc->allocate(1, 2) == 1);
  assert(loc->allocate(1, 3) == 2);
  assert(loc->freeMemory(2) == 1);
  assert(loc->allocate(3, 4) == 3);
  assert(loc->allocate(1, 1) == 1);
  assert(loc->allocate(1, 1) == 6);
  assert(loc->freeMemory(1) == 3);
  assert(loc->allocate(10, 2) == -1);
  assert(loc->freeMemory(7) == 0);
}