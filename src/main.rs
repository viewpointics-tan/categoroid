use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Arrow<'a> {
    PureArrow {
        name: &'a str,
        dom: &'a str,
        cod: &'a str,
    },
    Id {
        dom: &'a str,
        cod: &'a str,
    },
}
// struct Category {
//     arrows: Arrows,
//     context: Context,
// }

#[derive(Debug)]
struct Context<'a>(Vec<Arrow<'a>>);
impl<'a> Context<'a> {
    fn init(arrow: Arrow<'a>) -> Context<'a> {
        return Context(Vec::from([arrow]));
    }

    fn compose(mut self, arrow: Arrow<'a>) -> Context<'a> {
        self.0.push(arrow);
        return self;
    }

    //concat
}

fn main() {
    let arrows: HashSet<Arrow> = HashSet::from([Arrow::PureArrow {
        name: "f",
        dom: "A",
        cod: "B",
    }]);

    let context: Context = Context::init(Arrow::PureArrow {
        name: "f",
        dom: "A",
        cod: "B",
    })
    .compose(Arrow::PureArrow {
        name: "g",
        dom: "B",
        cod: "C",
    });

    println!("{:?}", context);
}
