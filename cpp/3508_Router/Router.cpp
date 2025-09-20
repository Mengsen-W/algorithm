#include <algorithm>
#include <cassert>
#include <deque>
#include <set>
#include <unordered_map>
#include <vector>

using namespace std;

class Router {
 public:
  int memLimit = 0;
  int length = 0;
  set<tuple<int, int, int>> isExist;
  unordered_map<int, deque<int>> sameDestQue;
  deque<tuple<int, int, int>> que;

  Router(int memoryLimit) { memLimit = memoryLimit; }

  bool addPacket(int source, int destination, int timestamp) {
    tuple<int, int, int> packet = make_tuple(source, destination, timestamp);
    if (isExist.contains(packet)) {
      return false;
    }
    if (length == memLimit) {
      forwardPacket();
    }
    length++;
    que.push_back(packet);
    sameDestQue[destination].push_back(timestamp);
    isExist.insert(packet);
    return true;
  }

  vector<int> forwardPacket() {
    vector<int> data;
    if (!que.empty()) {
      tuple<int, int, int> packet = que.front();
      que.pop_front();
      data = vector<int>{get<0>(packet), get<1>(packet), get<2>(packet)};
      isExist.erase(packet);
      sameDestQue[data[1]].pop_front();
      length--;
    }
    return data;
  }

  int getCount(int destination, int startTime, int endTime) {
    auto pos1 = lower_bound(sameDestQue[destination].begin(), sameDestQue[destination].end(), startTime);
    auto pos2 = upper_bound(sameDestQue[destination].begin(), sameDestQue[destination].end(), endTime);
    return pos2 - pos1;
  }
};

int main() {
  {
    Router router = Router(3);
    assert(router.addPacket(1, 4, 90) == true);
    assert(router.addPacket(2, 5, 90) == true);
    assert(router.addPacket(1, 4, 90) == false);
    assert(router.addPacket(3, 5, 95) == true);
    assert(router.addPacket(4, 5, 105) == true);
    assert((router.forwardPacket() == vector<int>{2, 5, 90}));
    assert(router.addPacket(5, 2, 110) == true);
    assert(router.getCount(5, 100, 110) == 1);
  }
  {
    Router router = Router(2);
    assert(router.addPacket(7, 4, 90) == true);
    assert((router.forwardPacket() == vector<int>{7, 4, 90}));
    assert((router.forwardPacket() == vector<int>{}));
  }
}