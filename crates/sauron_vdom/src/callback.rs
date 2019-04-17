use std::convert::Into;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Callback<IN, OUT>(Rc<dyn Fn(IN) -> OUT>);

impl<IN, F, OUT> From<F> for Callback<IN, OUT>
where
    F: Fn(IN) -> OUT + 'static,
{
    fn from(func: F) -> Self {
        Callback(Rc::new(func))
    }
}
impl<IN, OUT> fmt::Debug for Callback<IN, OUT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "||{{..}}")
    }
}

impl<IN, OUT> Callback<IN, OUT> {
    /// This method calls the actual callback.
    pub fn emit<T: Into<IN>>(&self, value: T) -> OUT {
        (self.0)(value.into())
    }
}

impl<IN, OUT> PartialEq for Callback<IN, OUT> {
    fn eq(&self, rhs: &Self) -> bool {
        Rc::ptr_eq(&self.0, &rhs.0)
    }
}
