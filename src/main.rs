mod diagram;

use diagram::TimingDiagram;

#[derive(Debug, Copy, Clone)]
struct Duration(u32);

impl From<&Duration> for u32 {
    fn from(value: &Duration) -> Self {
        value.0
    }
}

impl From<Duration> for u32 {
    fn from(value: Duration) -> Self {
        value.0
    }
}

impl From<&Duration> for i32 {
    fn from(value: &Duration) -> Self {
        value.0 as i32
    }
}

impl From<Duration> for i32 {
    fn from(value: Duration) -> Self {
        value.0 as i32
    }
}

#[derive(Debug, Clone)]
enum Value {
    V0,
    V1,
}

#[derive(Debug, Clone)]
struct State {
    pub value: Value,
    pub duration: Duration,
}

impl State {
    pub fn new(value: Value, duration: Duration) -> Self {
        Self { value, duration }
    }
}

struct Signal {
    name: Box<str>,
    states: Vec<State>,
}

fn main() {
    let mut diagram = TimingDiagram::new();
    let signal = Signal {
        name: "x".into(),
        states: vec![
            State::new(Value::V0, Duration(5)),
            State::new(Value::V1, Duration(5)),
            State::new(Value::V0, Duration(5)),
            State::new(Value::V0, Duration(5)),
            State::new(Value::V1, Duration(5)),
        ],
    };
    diagram.add_signal(signal);
    diagram.save("image.svg").expect("could not save diagram");
}
