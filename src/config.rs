// use crate::component::text::Text;
// use crate::component::traits::Component;
//
// #[derive(Debug)]
// pub struct Options {
//     pub separator: String,
//     pub background: String,
// }
//
// pub struct Config {
//     pub options: Options,
//     pub components: Vec<Box<dyn Component>>,
// }
//
// impl Default for Config {
//     fn default() -> Config {
//         Config {
//             options: Options {
//                 separator: "1".to_string(),
//                 background: "tr".to_string(),
//             },
//             components: vec![
//                 Box::new(Text::new("hello".to_string())),
//                 Box::new(Text::new("new".to_string())),
//             ],
//         }
//     }
// }
