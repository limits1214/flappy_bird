use bevy::prelude::*;
use bevy_flappy_bird::AppPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AppPlugin)
        .run();
}

struct Temp<'a, 'b, T> {
    aab: &'a str,
    bbb: &'b str,
    t: T,
}

fn asdf() -> () {
    let aaa = "";

    {
        let bbb = "";

        let t = Temp {
            aab: aaa,
            bbb: bbb,
            t: &String::new(),
        };
        temp(&t);
        // println!("{}", v);
    }
    ()
}

fn temp(t: &Temp<&String>) {
    println!("tmp t: {}", t.aab);
}

#[derive(Debug, Clone)]
struct St {}

fn lll() {
    let x = St {};
    let refx = &x;
    let y = refx.clone();
}

fn llla() {
    let x = 0;
    let x_ref = &x;

    let bb = x_ref;
    println!("{}", x_ref);
}

fn ww() {
    let mut x = 1;
    let y = &x;
    let z = *y;
    // x += z;

    let a = asddf();
}

fn asddf<'a>() -> &'a str {
    ""
}

// fn asdf22<'a>() -> &'a str {
//     let s = String::new();
//     &s
// }

fn aaaa() {
    let cc = String::from("value");
    let mmm = || {
        let sdf = cc;
    };
    mmm();

    let a = String::from("value");
    fnm(|| {
        let b = a.clone();
    });
}

fn fnm(mut f: impl FnMut() -> ()) {
    f();
}
