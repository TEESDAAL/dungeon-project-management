from random import randint
from math import sqrt
import pygame


WIN = pygame.display.set_mode((500, 500))
GRID_SIZE = 10


class Node:
    def __init__(self, value: int, x: int, y: int):
        self.x = x
        self.y = y
        self.value = value
        self.neighbors: list[Node] = []


class NodeNotFoundError(Exception):
    """Raised when a node is not found in the graph."""


class Graph:
    def __init__(self):
        self.nodes: set[Node] = set()
        self.edges: set[tuple[Node, Node]] = set()

    def add_node(self, node: Node):
        self.nodes.add(node)

    def add_edge(self, node1: Node, node2: Node):
        self.edges.add((node1, node2))

    def get_node(self, x: int, y: int) -> Node:
        for node in self.nodes:
            if node.x == x and node.y == y:
                return node
        raise NodeNotFoundError(f"Node {(x,y)} not in found")


graph = Graph()


def closest_node(nodes: set[Node], node: Node):
    return min(
        nodes - {node},
        key=lambda n: sqrt((n.x - node.x) ** 2 + (n.y - node.y) ** 2),
    )


def create_nodes(grid_size: int, num_nodes: int):
    locations: set[tuple[int, int]] = set()
    while num_nodes:
        x = randint(0, grid_size - 1)
        y = randint(0, grid_size - 1)
        if (x, y) not in locations:
            locations.add((x, y))
            graph.nodes.add(Node(num_nodes, x, y))
            num_nodes -= 1


NUMBER_NODES = round(GRID_SIZE ** (3 / 2))
# NUMBER_NODES = GRID_SIZE**2
create_nodes(GRID_SIZE, NUMBER_NODES)

unconnected_nodes = graph.nodes.copy()
visited_nodes: set[Node] = {unconnected_nodes.pop()}

while unconnected_nodes:
    closest_distance = float("inf")
    node_to_connect: Node | None = None
    for visited_node in visited_nodes:
        closest = closest_node(unconnected_nodes, visited_node)
        distance = sqrt(
            (closest.x - visited_node.x) ** 2
            + (closest.y - visited_node.y) ** 2
        )
        if distance < closest_distance:
            closest_distance = distance
            base_node = visited_node
            node_to_connect = closest

    if node_to_connect is None:
        break

    graph.add_edge(base_node, node_to_connect)
    unconnected_nodes.remove(node_to_connect)
    visited_nodes.add(node_to_connect)

# print(graph.nodes)
# print(graph.edges)
MULTIPLIER = 500 / GRID_SIZE

if __name__ == "__main__":
    clock = pygame.time.Clock()
    WIN.fill((255, 255, 255))
    for node in graph.nodes:
        pygame.draw.circle(
            WIN,
            (0, 0, 255),
            (node.x * MULTIPLIER + 25, node.y * MULTIPLIER + 25),
            20,
        )

    while True:
        clock.tick(60)
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                quit()
        for edge in graph.edges:
            pygame.draw.line(
                WIN,
                (0, 0, 0),
                (edge[0].x * MULTIPLIER + 25, edge[0].y * MULTIPLIER + 25),
                (edge[1].x * MULTIPLIER + 25, edge[1].y * MULTIPLIER + 25),
                3,
            )
        pygame.display.update()
