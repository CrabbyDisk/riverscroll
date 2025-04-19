use river_layout_toolkit::{GeneratedLayout, Layout, Rectangle, run};
use std::convert::Infallible;

fn main() {
    let layout = MyLayout::default();
    run(layout).unwrap();
}

#[derive(Default)]
struct MyLayout {
    offset: i32,
}

impl Layout for MyLayout {
    type Error = Infallible;

    const NAMESPACE: &'static str = "riverscroll";

    fn user_cmd(
        &mut self,
        cmd: String,
        _tags: Option<u32>,
        _output: &str,
    ) -> Result<(), Self::Error> {
        match cmd.as_str() {
            "forward" => self.offset += 1,
            "backward" => self.offset -= 1,
            _ => {}
        }
        Ok(())
    }

    fn generate_layout(
        &mut self,
        view_count: u32,
        usable_width: u32,
        usable_height: u32,
        _tags: u32,
        _output: &str,
    ) -> Result<GeneratedLayout, Self::Error> {
        // Wrap offset around to limit value.
        self.offset = self.offset.rem_euclid(view_count as i32);

        let views: Vec<_> = (0..view_count)
            .cycle()
            .skip(self.offset.try_into().unwrap())
            .take(view_count.try_into().unwrap())
            .map(|i| Rectangle {
                x: ((2 * usable_width / 3) * i) as i32,
                y: 0,
                width: 2 * usable_width / 3,
                height: usable_height,
            })
            .collect();

        Ok(GeneratedLayout {
            layout_name: "[]=".to_string(),
            views,
        })
    }
}
