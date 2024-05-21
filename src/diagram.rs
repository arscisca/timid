use std::cmp::Ordering;
use std::path::Path;

pub struct TimingDiagram {
    signals: Vec<Signal>,
}

impl TimingDiagram {
    pub fn new() -> Self {
        Self {
            signals: Vec::new(),
        }
    }

    pub fn signals(&self) -> &[Signal] {
        &self.signals
    }

    pub fn add_signal(&mut self, signal: Signal) {
        self.signals.push(signal)
    }

    pub fn render<P: AsRef<Path>>(&self, path: P) -> Result<(), anyhow::Error> {
        crate::render::render(self, path)
    }
}

pub type Timestamp = u64;

#[derive(Debug)]
pub struct Signal {
    name: Box<str>,
    states: Box<[State]>,
}

impl Signal {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn states(&self) -> &[State] {
        &self.states
    }

    pub fn sample(&self, t: Timestamp) -> &Value {
        match self
            .states
            .binary_search_by_key(&t, |state| state.timestamp)
        {
            Ok(i) => &self.states[i].value,
            Err(i) => &self.states[i].value,
        }
    }
}

pub struct SignalBuilder {
    name: Box<str>,
    states: std::collections::BinaryHeap<StateOrd>,
}

impl SignalBuilder {
    pub fn new<S: Into<Box<str>>>(name: S) -> Self {
        Self {
            name: name.into(),
            states: Default::default(),
        }
    }

    pub fn with_states<I: IntoIterator<Item = State>>(mut self, states: I) -> Self {
        let iter = states.into_iter();
        self.states.reserve(iter.size_hint().0);
        for state in iter {
            self.add_state(state);
        }
        self
    }

    pub fn add_state(&mut self, state: State) {
        let ord = StateOrd(state);
        self.states.push(ord)
    }

    pub fn build(self) -> Signal {
        self.into()
    }
}

impl From<SignalBuilder> for Signal {
    fn from(value: SignalBuilder) -> Self {
        let states: Vec<State> = value
            .states
            .into_sorted_vec()
            .into_iter()
            .map(Into::into)
            .collect();
        Self {
            name: value.name,
            states: states.into(),
        }
    }
}

struct StateOrd(State);

impl PartialEq for StateOrd {
    fn eq(&self, other: &Self) -> bool {
        self.0.timestamp.eq(&other.0.timestamp)
    }
}

impl Eq for StateOrd {}

impl PartialOrd for StateOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StateOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.timestamp.cmp(&other.0.timestamp)
    }
}

impl From<StateOrd> for State {
    fn from(value: StateOrd) -> Self {
        value.0
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    V0,
    V1,
    VX,
    VZ,
}

#[derive(Debug, Clone)]
pub struct State {
    pub value: Value,
    pub timestamp: Timestamp,
}

impl State {
    pub fn new(value: Value, timestamp: Timestamp) -> Self {
        Self { value, timestamp }
    }
}
