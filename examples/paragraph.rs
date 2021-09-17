use witui::widgets::text::{Text, TextAlign};
use witui::widgets::Borders;
use witui::Style;
use witui::WiTui;

fn main() {
  let text = Text::new("\
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Vestibulum vehicula tincidunt metus, eget consequat metus iaculis vel. Integer sit amet sodales eros, non pretium mauris. Curabitur eleifend facilisis augue a elementum.
Etiam suscipit bibendum odio et lobortis. Mauris facilisis accumsan tortor vitae aliquet. Mauris ultricies pulvinar felis eu placerat.
In sed pharetra tortor, ut condimentum nunc.
Donec porta, ex eget varius ultrices, sapien enim iaculis lorem, id sollicitudin orci tellus id dui. Curabitur tincidunt rutrum auctor. Maecenas non suscipit nisi, id porta turpis. Vivamus faucibus finibus ligula, non mattis enim convallis at. Curabitur dictum lacus ut diam luctus, at tempor eros eleifend. Fusce nunc ligula, rhoncus et pellentesque hendrerit, sagittis quis risus. Vivamus malesuada diam id augue tristique, at suscipit magna efficitur. Integer scelerisque condimentum tortor, eu euismod purus blandit sed. In gravida vel purus ut blandit. Aliquam ut congue nisl. Cras vitae purus convallis, vehicula ligula et, dapibus turpis. Curabitur placerat aliquam ex non blandit. Vestibulum vestibulum urna ornare purus aliquet, sit amet suscipit lacus fermentum. Integer consequat est sed placerat congue."
  ).align(TextAlign::Justify);

  let root = Borders::with_child(text).borders_double(Style::default());

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
