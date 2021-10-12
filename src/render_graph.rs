use crate::dot::{font_tag, html_escape, DotGraph};
use bevy::{
    reflect::TypeRegistration,
    render::render_graph::{Edge, NodeId, RenderGraph},
};
use crate::zip_longest::{EitherOrBoth, zip_longest};

/// Formats the render graph into a dot graph.
pub fn render_graph_dot(graph: &RenderGraph) -> String {
    let default_style = RenderGraphStyle::light();
    render_graph_dot_styled(graph, &default_style)
}
pub struct RenderGraphStyle {
    pub fontname: String,
    pub fontsize: f32,
    pub textcolor: String,
    pub typename_color: String,
    pub background_color: String,
    pub node_color: String,
    pub node_style: String,
    pub edge_color: String,
    pub slot_edge_color: String,
}
impl RenderGraphStyle {
    pub fn light() -> Self {
        RenderGraphStyle {
            fontname: "Helvetica".into(),
            fontsize: 14.0,
            textcolor: "black".into(),
            typename_color: "red".into(),
            background_color: "white".into(),
            node_color: "black".into(),
            node_style: "rounded".into(),
            edge_color: "black".into(),
            slot_edge_color: "blue".into(),
        }
    }

    pub fn dark() -> Self {
        RenderGraphStyle {
            fontname: "Helvetica".into(),
            fontsize: 14.0,
            textcolor: "white".into(),
            typename_color: "red".into(),
            background_color: "#35393F".into(),
            node_color: "#99aab5".into(),
            node_style: "rounded".into(),
            edge_color: "white".into(),
            slot_edge_color: "white".into(),
        }
    }
}

/// Formats the render graph into a dot graph using a custom [RenderGraphStyle].
pub fn render_graph_dot_styled(graph: &RenderGraph, style: &RenderGraphStyle) -> String {
    let options = [("rankdir", "LR"), ("ranksep", "1.0")];

    // Convert to format fitting GraphViz node id requirements
    let node_id = |id: &NodeId| format!("{}", id.uuid().as_u128());
    let mut dot = DotGraph::digraph("RenderGraph", &options)
        .graph_attributes(&[("bgcolor", &style.background_color)])
        .edge_attributes(&[
            ("fontname", &style.fontname),
            ("fontcolor", &style.textcolor),
        ])
        .node_attributes(&[
            ("shape", "plaintext"),
            ("fontname", &style.fontname),
            ("fontcolor", &style.textcolor),
        ]);

    let mut nodes: Vec<_> = graph.iter_nodes().collect();
    nodes.sort_by_key(|node_state| &node_state.type_name);

    for node in &nodes {
        let name = node.name.as_deref().unwrap_or("<node>");
        let type_name = TypeRegistration::get_short_name(node.type_name);

        let inputs = node
            .input_slots
            .iter()
            .enumerate()
            .map(|(index, slot)| {
                format!(
                    "<TD PORT=\"{}\">{}: {}</TD>",
                    html_escape(&format!("{}", index)),
                    html_escape(&slot.info.name),
                    html_escape(&format!("{:?}", slot.info.resource_type))
                )
            })
            .collect::<Vec<_>>();

        let outputs = node
            .output_slots
            .iter()
            .enumerate()
            .map(|(index, slot)| {
                format!(
                    "<TD PORT=\"{}\">{}: {}</TD>",
                    html_escape(&format!("{}", index)),
                    html_escape(&slot.info.name),
                    html_escape(&format!("{:?}", slot.info.resource_type))
                )
            })
            .collect::<Vec<_>>();

        let slots = zip_longest(inputs.iter(), outputs.iter())
            .map(|pair| match pair {
                EitherOrBoth::Both(input, output) => format!("<TR>{}{}</TR>", input, output),
                EitherOrBoth::Left(input) => {
                    format!("<TR>{}<TD BORDER=\"0\">&nbsp;</TD></TR>", input)
                }
                EitherOrBoth::Right(output) => {
                    format!("<TR><TD BORDER=\"0\">&nbsp;</TD>{}</TR>", output)
                }
            })
            .collect::<String>();

        let label = format!(
            "<<TABLE STYLE=\"rounded\"><TR><TD PORT=\"title\" BORDER=\"0\" COLSPAN=\"2\">{}<BR/>{}</TD></TR>{}</TABLE>>",
            html_escape(name),
            font_tag(&type_name, &style.typename_color, 10),
            slots,
        );

        dot.add_node(
            &node_id(&node.id),
            &[
                ("label", &label),
                ("color", &style.node_color),
                ("fillcolor", &style.node_color),
            ],
        );
    }

    for node in graph.iter_nodes() {
        for edge in &node.edges.input_edges {
            match edge {
                Edge::SlotEdge {
                    input_node,
                    input_index,
                    output_node,
                    output_index,
                } => {
                    dot.add_edge_with_ports(
                        &node_id(output_node),
                        Some(&format!("{}:e", output_index)),
                        &node_id(input_node),
                        Some(&format!("{}:w", input_index)),
                        &[("color", &style.slot_edge_color)],
                    );
                }
                Edge::NodeEdge {
                    input_node,
                    output_node,
                } => {
                    dot.add_edge_with_ports(
                        &node_id(output_node),
                        Some("title:e"),
                        &node_id(input_node),
                        Some("title:w"),
                        &[("color", &style.edge_color)],
                    );
                }
            }
        }
    }

    dot.finish()
}
