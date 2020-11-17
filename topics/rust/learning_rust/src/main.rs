fn main() -> () {
    let e1 = Effect::A1;
    let e2 = Effect::A2;
    let e3 = Effect::B;

    do_this(e1).unwrap();
    do_this(e2).unwrap();
    do_this(e3).unwrap();
}

pub enum Effect {
    A1,
    A2,
    B
}

enum AError {}
enum BError {}
#[derive(Debug)]
pub enum PublicError {}

impl From<AError> for PublicError {
    fn from(e: AError) -> PublicError {
        PublicError{}
    }
}

impl From<BError> for PublicError {
    fn from(e: BError) -> PublicError {
        PublicError{}
    }
}

fn doA1() -> Result<(), AError> {
    Ok(())
}
fn doA2() -> Result<(), AError> {
    Ok(())
}
fn doB() -> Result<(), AError> {
    Ok(())
}

pub fn do_this(e: Effect) -> Result<(), PublicError> {
    match e {
        Effect::A1 => doA1()?,
        Effect::A2 => doA2()?,
        Effect::B => doB()?,
    };
    Ok(())
}
