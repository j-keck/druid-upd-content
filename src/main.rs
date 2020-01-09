use druid::{
    self,
    lens::Lens,
    widget::*,
    AppLauncher, Data, Env, Widget, WindowDesc,
};
use std::{
    thread,
    time::Duration,
    sync::{Arc, RwLock},
};

#[derive(Debug, Data, Clone, Lens)]
struct State {
    things: Arc<Things>,
}

fn main() {
    let things = Arc::new(Things::default());
    things.add(Thing::Text("I'm a member of the list".to_string()));
    things.add(Thing::Num(1));

    AppLauncher::with_window(WindowDesc::new(ui_builder))
        .launch(State { things })
        .unwrap();
}

fn ui_builder() -> impl Widget<State> {
    Flex::column()
        .with_child(
            Button::new("Add 5 elements to the list", |_, state: &mut State, _| {
                let state = state.clone();
                thread::spawn(move || {
                    let n = state.things.data_len() as i32;
                    for i in n..(n + 5) {
                        println!("add {} to state", i);
                        state.things.add(Thing::Num(i));
                        thread::sleep(Duration::from_millis(500));
                    }
                });
            }), 0.1)

        .with_child(Label::new(|state: &State, _: &Env|
                               format!("Number of elements in the list: {}", state.things.data_len()))
                    .padding(5.0)
                    .center(),
                    0.0)

        .with_child(
            Scroll::new(List::new(|| {
                Label::new(|t: &Thing, _: &Env| {
                    format!("{:?}", t)
                })
                    .padding(5.0)
            }))
                .vertical()
                .lens(State::things),
            1.0)
}


#[derive(Debug, Data, Clone)]
enum Thing {
    Text(String),
    Num(i32),
}

#[derive(Debug, Default)]
struct Things(RwLock<Vec<Thing>>);
impl Things {
    pub fn add(&self, t: Thing) {
        self.0.write().unwrap().push(t)
    }
}

impl druid::widget::ListIter<Thing> for std::sync::Arc<Things> {
    fn for_each(&self, mut cb: impl FnMut(&Thing, usize)) {
        for (n, t) in self.0.read().unwrap().iter().enumerate() {
            println!("ListIter::for_each - n: {}, t: {:?}", n, t);
            cb(t, n)
        }
    }
    fn for_each_mut(&mut self, mut _cb: impl FnMut(&mut Thing, usize)) {
    }
    fn data_len(&self) -> usize {
        self.0.read().unwrap().len()
    }
}

