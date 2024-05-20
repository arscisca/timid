use std::fmt::Write;
use std::path::Path;
use svg::{node::element, Node};

use crate::{Signal, State, Value};

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

    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let renderer = Renderer::new(self);
        renderer.render(path)
    }
}

struct Renderer<'d> {
    diagram: &'d TimingDiagram,
    svg: element::SVG,
}

impl<'d> Renderer<'d> {
    const WAVE_HEIGHT: i32 = 10;

    pub fn new(diagram: &'d TimingDiagram) -> Self {
        Self {
            diagram,
            svg: element::SVG::new(),
        }
    }

    pub fn render<P: AsRef<Path>>(mut self, path: P) -> std::io::Result<()> {
        self.svg.assign("viewBox", (0, 0, 100, 100));
        let signals = self.diagram.signals();
        let waves = self.render_waves(signals)
            .set("transform", "translate(10, 20)");
        self.svg.append(waves);
        svg::save(path, &self.svg)
    }

    fn render_waves(&self, signals: &[Signal]) -> element::Group {
        let mut waves = element::Group::new();
        waves.assign("id", "waves");
        waves.append(
            element::Style::new("\
                .wave {\
                    fill: none; \
                    stroke: red; \
                    stroke-width: 0.5;\
                }\
                .wave:hover{\
                    stroke: black;\
                }")
        );
        for signal in signals {
            let wave = self.render_wave(signal)
                .set("class", "wave")
                .set("transform", "scale(1, -1)");
            waves.append(wave);
        }
        waves
    }

    fn render_wave(&self, signal: &Signal) -> element::Group {
        let mut t = 0;
        let mut group = element::Group::new().set("id", format!("wave:{}", signal.name));
        if signal.states.is_empty() {
            return group;
        }
        // Draw the first value.
        if let Some(first) = signal.states.first() {
            group.append(self.render_state(t, first));
            let dt: i32 = first.duration.into();
            t += dt;
        }
        for window in signal.states.windows(2) {
            let (prev, curr) = (&window[0], &window[1]);
            group.append(self.render_state_transition(t, &prev.value, &curr.value));
            group.append(self.render_state(t, curr));
            let dt: i32 = curr.duration.into();
            t += dt;
        }
        group
    }

    fn render_state(&self, t: i32, state: &State) -> Box<dyn Node> {
        match &state.value {
            v @ (Value::V0 | Value::V1) => {
                let y = match v {
                    Value::V0 => 0,
                    Value::V1 => Self::WAVE_HEIGHT,
                };
                let dt: i32 = state.duration.into();
                let data = element::path::Data::new().move_to((t, y)).line_by((dt, 0));
                let path = element::Path::new()
                    .set("d", data);
                Box::new(path)
            }
        }
    }

    fn render_state_transition(&self, t: i32, v1: &Value, v2: &Value) -> Box<dyn Node> {
        match (v1, v2) {
            (Value::V0, Value::V0) | (Value::V1, Value::V1) => {
                // No transition here.
                Box::new(element::Path::new())
            }
            (Value::V0, Value::V1) => {
                let data = element::path::Data::new()
                    .move_to((t, 0))
                    .line_by((0, Self::WAVE_HEIGHT));
                let path = element::Path::new()
                    .set("d", data);
                Box::new(path)
            }
            (Value::V1, Value::V0) => {
                let data = element::path::Data::new()
                    .move_to((t, Self::WAVE_HEIGHT))
                    .line_by((0, -Self::WAVE_HEIGHT));
                let path = element::Path::new()
                    .set("d", data);
                Box::new(path)
            }
            (v1, v2) => {
                panic!("unsupported state transition from {:?} to {:?}", v1, v2);
            }
        }
    }
}
