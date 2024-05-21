use std::path::Path;
use svg::{node::element, Node};

use crate::diagram::{Signal, State, TimingDiagram, Value};

type Coordinate = i32;

pub struct RendererSvg<'d> {
    diagram: &'d TimingDiagram,
    svg: element::SVG,
}

impl<'d> RendererSvg<'d> {
    pub fn new(diagram: &'d TimingDiagram) -> Self {
        Self {
            diagram,
            svg: element::SVG::new(),
        }
    }

    fn render_waves(&self, signals: &[Signal]) -> element::Group {
        let mut waves = element::Group::new();
        waves.assign("id", "waves");
        waves.append(element::Style::new(
            "\
                .wave {\
                    fill: none; \
                    stroke: black; \
                    stroke-width: 0.5;\
                }\
                .wave:hover{\
                    stroke: black;\
                }",
        ));
        for signal in signals {
            let mut wave_planner = WavePlanner::default();
            for state in signal.states() {
                wave_planner.add_state(state);
            }
            let wave = wave_planner
                .render()
                .set("class", "wave")
                .set("transform", "scale(1, -1)");
            waves.append(wave);
        }
        waves
    }
}

impl<'d> super::Renderer for RendererSvg<'d> {
    type Error = SvgError;

    fn render(mut self, path: &Path) -> Result<(), Self::Error> {
        self.svg.assign("viewBox", (0, 0, 100, 100));
        let signals = self.diagram.signals();
        let waves = self
            .render_waves(signals)
            .set("transform", "translate(10, 20)");
        self.svg.append(waves);
        svg::save(path, &self.svg)?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SvgError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Planner for a wave with binary states.
struct WavePlanner {
    data: element::path::Data,
    x: Coordinate,
    y: Coordinate,
    transition_time: Coordinate,
    first: bool,
}

impl WavePlanner {
    const WAVE_HEIGHT: Coordinate = 2;
    const WAVE_CENTER: Coordinate = Self::WAVE_HEIGHT / 2;

    pub fn new(transition_time: Coordinate) -> Self {
        Self {
            data: element::path::Data::new(),
            x: 0,
            y: 0,
            transition_time,
            first: true,
        }
    }

    pub fn add_state(&mut self, state: &State) {
        let y = self.height(&state.value);
        if self.first {
            self.move_to(0, y);
            self.first = false;
        }
        let dx: Coordinate = state.timestamp as Coordinate - self.x;
        let dy: Coordinate = y - self.y;
        self.line_by(dx, 0);
        self.line_by(self.transition_time, dy);
    }

    pub fn render(self) -> element::Path {
        element::Path::new().set("d", self.data)
    }

    fn height(&self, value: &Value) -> Coordinate {
        match value {
            Value::V0 => 0,
            Value::V1 => Self::WAVE_HEIGHT,
            v => panic!("value {:?} not supported", v),
        }
    }

    fn update_pos(&mut self, x: Coordinate, y: Coordinate) {
        self.x = x;
        self.y = y;
    }

    fn move_to(&mut self, x: Coordinate, y: Coordinate) {
        self.data = std::mem::take(&mut self.data).move_to((x, y));
        self.update_pos(x, y);
    }

    fn line_by(&mut self, dx: Coordinate, dy: Coordinate) {
        self.data = std::mem::take(&mut self.data).line_by((dx, dy));
        self.update_pos(self.x + dx, self.y + dy);
    }
}

impl Default for WavePlanner {
    fn default() -> Self {
        Self::new(1)
    }
}
