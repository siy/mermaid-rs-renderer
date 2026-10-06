use mermaid_rs_renderer::{RenderOptions, parse_mermaid_strict, render_with_options};

fn render(input: &str) -> String {
    render_with_options(input, RenderOptions::mermaid_default()).expect("valid diagram")
}

#[test]
fn public_api_accepts_continuations_but_not_missing_sources() {
    let input = "flowchart LR\n A[Source]\n %% comment\n -->|read| B[Viewer]\n --> C[Done]\n";
    let parsed = parse_mermaid_strict(input).unwrap();
    assert_eq!(parsed.graph.nodes.len(), 3);
    assert_eq!(parsed.graph.edges.len(), 2);
    assert_eq!(parsed.graph.edges[0].from, "A");
    assert_eq!(parsed.graph.edges[1].from, "B");
    assert!(render(input).contains("Viewer"));
    for invalid in [
        "flowchart LR\n --> B",
        "flowchart LR\n A;\n --> B",
        "flowchart LR\n subgraph S\n --> B\n end",
        "flowchart LR\n A\n -->",
    ] {
        assert!(
            render_with_options(invalid, RenderOptions::default()).is_err(),
            "{invalid}"
        );
    }
}
