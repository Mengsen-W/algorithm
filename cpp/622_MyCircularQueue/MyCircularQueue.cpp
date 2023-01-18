/*
 * @Date: 2022-08-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-02
 * @FilePath: /algorithm/622_MyCircularQueue/MyCircularQueue.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class MyCircularQueue {
 private:
  int front;
  int rear;
  int capacity;
  vector<int> elements;

 public:
  MyCircularQueue(int k) {
    this->capacity = k + 1;
    this->elements = vector<int>(capacity);
    rear = front = 0;
  }

  bool enQueue(int value) {
    if (isFull()) {
      return false;
    }
    elements[rear] = value;
    rear = (rear + 1) % capacity;
    return true;
  }

  bool deQueue() {
    if (isEmpty()) {
      return false;
    }
    front = (front + 1) % capacity;
    return true;
  }

  int Front() {
    if (isEmpty()) {
      return -1;
    }
    return elements[front];
  }

  int Rear() {
    if (isEmpty()) {
      return -1;
    }
    return elements[(rear - 1 + capacity) % capacity];
  }

  bool isEmpty() { return rear == front; }

  bool isFull() { return ((rear + 1) % capacity) == front; }
};

int main() {
  MyCircularQueue obj{3};
  assert(obj.enQueue(1) == true);
  assert(obj.enQueue(2) == true);
  assert(obj.enQueue(3) == true);
  assert(obj.enQueue(4) == false);
  assert(obj.Rear() == 3);
  assert(obj.isFull() == true);
  assert(obj.deQueue() == true);
  assert(obj.enQueue(4) == true);
  assert(obj.Rear() == 4);
  assert(obj.Front() == 2);
}