macro_rules! with_financials {
    ($(#[$m:meta])*
    struct $name:ident {
        $($field:ident : $f_ty:ty),* $(,)?
    }) => {
        $(#[$m])*
        struct $name {
            x: i32,
            y: u64,
            $($field : $f_ty),*
        }
    }
}

with_financials! {
    #[derive(Debug)]
    struct Lol {
        lol: f32,
    }
}

with_financials! {
    #[derive(Debug)]
    struct Haha {
        ha: &'static str,
    }
}

fn main() {
    let a = Lol {
        x: 5,
        y: 10,
        lol: 3.2,
    };
    let b = Haha {
        x: 2,
        y: 3,
        ha: "wahaha!",
    };
    dbg!(&a, &b);
}
