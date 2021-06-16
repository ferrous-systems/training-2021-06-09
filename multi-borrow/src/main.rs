use std::collections::HashMap;

struct RelevantContext;

impl RelevantContext {
    fn do_trigger(&mut self, trig: &str) {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Trigger {
    id: u64,
    // other stuff here
}

struct Contrived {
    triggers: HashMap<Trigger, String>,
    context: RelevantContext,
}

fn do_the_thing(trig: &Trigger, context: &mut RelevantContext) {
    todo!()
}

impl Contrived {
    fn some_processing(&mut self) {
        // Gather IDs from some collection thing, but fake it here
        let ids = vec![8086, 68000];

        // Irrefutable Binding
        {
            let Contrived {
                triggers,  // triggers: &mut HashMap<...>
                context,   // context: &mut RelevantContext
                ..
            } = self;

            let triggers = &mut self.triggers;
            let context = &mut self.context;

            // Find the viable components
            let relevant_ids = ids
                .iter()
                .map(|i| Trigger { id: *i } )
                .for_each(|t| {
                    if let Some(r) = triggers.get_mut(&t) {
                        context.do_trigger(&r);
                    }
                });
        }

        self.whatever();

    }
}

struct Point {
    x: i8,
    y: i8,
}

fn main() {
    let mut example = Contrived {
        triggers: HashMap::new(),
        context: RelevantContext,
    };

    let x = vec![(1u8, "foo", -1i64)];

    // Irrefutable Bindings
    for (a, b, c) in x.iter() {

    }

    let y = vec![Point { x: 1, y: 1 }];

    // Irrefutable Bindings
    for Point { x: a, y: b } in y.iter() {
        println!("{}", a)
    }

    example.some_processing();
}
