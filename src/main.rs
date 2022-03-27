#[derive(Debug)]
enum Arrow {
    PureArrow {
        name: String,
        dom: String,
        cod: String,
    },
    Id {
        dom: String,
        cod: String,
    },
}
// struct Category {
//     arrows: Arrows,
//     context: Context,
// }

#[derive(Debug)]
struct Context(Vec<Arrow>);
impl Context {
    fn new() -> Context {
        let new_vec: Vec<Arrow> = vec![];
        return Context(new_vec);
    }

    fn init(&mut self, arrow: Arrow) -> &mut Context {
        self.0 = vec![arrow];
        return self;
    }

    fn compose(&mut self, arrow: Arrow) -> &mut Context {
        self.0.push(arrow);
        return self;
    }

    //concat
}

fn main() {
    let arrows: Vec<Arrow> = vec![];

    let mut context: Context = Context::new();
    context
        .init(Arrow::PureArrow {
            name: "f".to_string(),
            dom: "A".to_string(),
            cod: "B".to_string(),
        })
        .compose(Arrow::PureArrow {
            name: "g".to_string(),
            dom: "B".to_string(),
            cod: "C".to_string(),
        });

    println!("{:?}", context);
}
