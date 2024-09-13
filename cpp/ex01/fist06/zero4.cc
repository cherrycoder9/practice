// shared_ptr을 이용한 그래프 구현
// 노드의 동적 할당과 해제를 포함한 그래프 구성

#include <iostream>
#include <memory>
#include <queue>
#include <unordered_set>
#include <vector>

using namespace std;

// Node 클래스는 그래프의 각 노드를 나타냄
class Node {
   public:
    int data;  // 노드가 저장할 정수형 데이터
    vector<shared_ptr<Node>> neighbors;  // 인접 노드 리스트

    // 생성자: 노드를 생성할 때 데이터를 입력받아 초기화
    Node(int val) : data(val) {
    }
};

// Graph 클래스는 노드들 간의 연결과 탐색을 관리함
class Graph {
   public:
    // 두 노드 간의 간선을 추가하는 함수
    void addEdge(shared_ptr<Node> u, shared_ptr<Node> v) {
        // u 노드의 이웃 리스트에 v 노드를 추가
        u->neighbors.push_back(v);
        // v 노드의 이웃 리스트에도 u 노드를 추가 (양방향 그래프)
        v->neighbors.push_back(u);
    }

    // BFS 알고리즘을 이용해 그래프를 출력하는 함수
    void printGraph(shared_ptr<Node> startNode) {
        // 방문한 노드를 기록하기 위한 집합 (중복 방지)
        unordered_set<shared_ptr<Node>> visited;

        // BFS를 위한 큐 (탐색할 노드 목록)
        queue<shared_ptr<Node>> q;

        // 시작 노드를 큐에 추가하고 방문 표시
        q.push(startNode);
        visited.insert(startNode);

        // 큐가 빌 때까지 BFS를 수행
        while (!q.empty()) {
            // 큐의 맨 앞에 있는 노드를 가져옴
            shared_ptr<Node> current = q.front();
            q.pop();  // 큐에서 해당 노드를 제거

            // 현재 노드의 데이터를 출력
            cout << "노드 " << current->data << "와 연결된 노드들: ";

            //  현재 노드의 이웃 노드를 모두 탐색
            for (auto& neighbor : current->neighbors) {
                cout << neighbor->data << " ";  // 이웃 노드 데이터 출력

                // 아직 방문하지 않은 이웃 노드라면
                if (visited.find(neighbor) == visited.end()) {
                    // 이웃 노드를 큐에 추가하고 방문 표시
                    q.push(neighbor);
                    visited.insert(neighbor);
                }
            }
            cout << endl;  // 다음 노드로 넘어가기 전에 줄바꿈
        }
    }
};

int main() {
    // 각 노드를 동적으로 할당하고 스마트 포인터로 관리
    shared_ptr<Node> node1 = make_shared<Node>(1);
    shared_ptr<Node> node2 = make_shared<Node>(2);
    shared_ptr<Node> node3 = make_shared<Node>(3);
    shared_ptr<Node> node4 = make_shared<Node>(4);
    shared_ptr<Node> node5 = make_shared<Node>(5);

    // 그래프 객체 생성
    Graph graph;

    // 그래프에 간선 추가 (노드들 연결)
    graph.addEdge(node1, node2);
    graph.addEdge(node1, node3);
    graph.addEdge(node2, node4);
    graph.addEdge(node3, node5);

    // BFS 방식으로 그래프 출력 (노드 1부터 탐색 시작)
    cout << "그래프 탐색 결과: " << endl;
    graph.printGraph(node1);

    return 0;
}