use super::*;

// Connection Saturation Rate
pub fn spawn_conn_satur_rate(parent: &mut ChildBuilder) {
    parent.spawn((Text::new("CSR:"),));
}
