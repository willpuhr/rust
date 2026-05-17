const A: () = {
    return Ok(()); //~ ERROR return statement outside of function body
    ()
};

fn main() -> Result<(), ()> {
    const B: () = {
        return Ok(()); //~ ERROR cannot return from inside a `const` initializer
        ()
    };

    static C: () = {
        return Ok(()); //~ ERROR cannot return from inside a `static` initializer
        ()
    };

    Ok(())
}
