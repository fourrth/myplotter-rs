pub use plotly;
use plotly::{color::NamedColor, common::Line, layout::AxisRange};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlottingError {
    #[error("Invalid length of vec")]
    InvalidLength,
}

/// plot data_xys which is (x,y,'trace name')
pub fn plot_xys<
    X: serde_core::Serialize + Clone + 'static,
    Y: serde_core::Serialize + Clone + 'static,
>(
    data_xys: impl ExactSizeIterator<Item = (Vec<X>, Vec<Y>, String)>,
    title: &str,
    x_label: &str,
    x_window: Option<AxisRange>,
    y_label: &str,
    y_window: Option<AxisRange>,
) -> Result<plotly::Plot, PlottingError> {
    let l = data_xys.len();
    if l == 0 {
        return Err(PlottingError::InvalidLength);
    }

    let traces = data_xys.into_iter().map(|(x, y, name)| {
        plotly::Scatter::new(x.clone(), y.clone())
            .mode(plotly::common::Mode::Lines)
            .name(name)
    });

    let layout = plotly::Layout::new()
        .title(title)
        .x_axis({
            let mut axis = plotly::layout::Axis::new().title(x_label);
            if let Some(xr) = x_window {
                axis = axis.range(xr).auto_range(false);
            } else {
                axis = axis.auto_range(true);
            }
            axis
        })
        .y_axis({
            let mut axis = plotly::layout::Axis::new().title(y_label);
            if let Some(yr) = y_window {
                axis = axis.range(yr).auto_range(false);
            } else {
                axis = axis.auto_range(true);
            }
            axis
        });

    let mut plot = plotly::Plot::new();

    let config = plotly::Configuration::new()
        .responsive(true)
        .double_click(plotly::configuration::DoubleClick::ResetAutoSize)
        .scroll_zoom(true);

    plot.set_configuration(config);
    plot.set_layout(layout);

    for (cx, trace) in traces.into_iter().enumerate() {
        let line = match cx % 5 {
            0 => Line::new().color(NamedColor::Red),
            1 => Line::new().color(NamedColor::Blue),
            2 => Line::new().color(NamedColor::Green),
            3 => Line::new().color(NamedColor::Orange),
            4 => Line::new().color(NamedColor::Violet),
            _ =>
            /* impossible */
            {
                Line::new().color(NamedColor::Black)
            }
        };
        plot.add_trace(trace.line(line));
    }

    Ok(plot)
}
