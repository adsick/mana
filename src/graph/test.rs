use super::*;

#[test]
fn graph_building() {
    let mut graph: Graph<char> = Graph::new('\0');

    let root = graph.build();

    root.add((), 'a').add((), 'b');

    let node_d = root.add((), 'c').add((), 'd');

    let node_e = node_d.add((), 'e');
    let node_f = node_d.add((), 'f');
    let node_g = node_d.add((), 'g');

    node_e.looping(());

    node_f.link(&node_g, ());
    node_g.link(&node_f, ());

    dbg!(&graph);

    // now let's create the same graph without builder

    let mut root = Node::new('\0'); // 0
    let mut a = Node::new('a'); // 1
    root.add_edge(1, ());

    let b = Node::new('b'); // 2
    a.add_edge(2, ());

    let mut c = Node::new('c'); // 3
    root.add_edge(3, ());

    let mut d = Node::new('d'); // 4
    c.add_edge(4, ());

    let mut e = Node::new('e'); // 5
    let mut f = Node::new('f'); // 6
    let mut g = Node::new('g'); // 7

    d.add_edge(5, ());
    d.add_edge(6, ());
    d.add_edge(7, ());

    e.add_edge(5, ());

    f.add_edge(7, ());
    g.add_edge(6, ());

    let expected = Graph {
        nodes: vec![root, a, b, c, d, e, f, g],
    };

    assert_eq!(graph, expected);
}
