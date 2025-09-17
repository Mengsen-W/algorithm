#include <cassert>
#include <queue>
#include <unordered_map>
#include <vector>

using namespace std;

class NumberContainers {
 private:
  unordered_map<int, int> nums;
  unordered_map<int, priority_queue<int, vector<int>, greater<>>> heaps;

 public:
  NumberContainers() {}

  void change(int index, int number) {
    nums[index] = number;
    heaps[number].push(index);
  }

  int find(int number) {
    while (!heaps[number].empty() && nums[heaps[number].top()] != number) {
      heaps[number].pop();
    }
    if (heaps[number].empty()) {
      return -1;
    }
    return heaps[number].top();
  }
};

int main() {
  NumberContainers nc = NumberContainers();
  assert(nc.find(10) == -1);
  nc.change(2, 10);
  nc.change(1, 10);
  nc.change(3, 10);
  nc.change(5, 10);
  assert(nc.find(10) == 1);
  nc.change(1, 20);
  assert(nc.find(10) == 2);
}
