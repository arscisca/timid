mod diagram;
mod render;

use diagram::{SignalBuilder, State, TimingDiagram, Value};

fn main() {
    let mut diagram = TimingDiagram::new();
    let signal = SignalBuilder::new("x")
        .with_states(vec![
            State::new(Value::V0, 0),
            State::new(Value::V1, 3),
            State::new(Value::V0, 15),
            State::new(Value::V1, 50),
        ])
        .build();
    diagram.add_signal(signal);
    diagram.render("image.svg").expect("could not save diagram");
}
