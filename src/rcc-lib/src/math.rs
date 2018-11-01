use crate::{RccDouble as Double, RccInt as Int};

macro_rules! double_defs {
    ($($name: ident )+) => {
        $(
            #[no_mangle]
            pub extern "C" fn $name(x: Double) -> Double {
                Double::$name(x)
            }
        )+
    };
}

double_defs! {
    acos asin atan cos cosh sin sinh tanh
    exp log10 sqrt ceil floor
}

#[no_mangle]
pub extern "C" fn atan2(x: Double, y: Double) -> Double {
    Double::atan2(x, y)
}

#[no_mangle]
pub extern "C" fn pow(x: Double, y: Double) -> Double {
    x.powf(y)
}

#[no_mangle]
pub extern "C" fn fabs(x: Double) -> Double {
    x.abs()
}

#[no_mangle]
pub extern "C" fn log(x: Double) -> Double {
    x.ln()
}

#[no_mangle]
pub extern "C" fn fmod(x: Double, y: Double) -> Double {
    x.mod_euc(y)
}

#[no_mangle]
pub extern "C" fn frexp(_x: Double, _exponent: *const Int) -> Double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn ldexp(_x: Double, _exponent: Int) -> Double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn modf(_x: Double, _integer: *const Double) -> Double {
    unimplemented!()
}

/*

// extern double acos(double x);
// extern double asin(double x);
// extern double atan(double x);
// extern double atan2(double y, double x);
// extern double cos(double x);
// extern double cosh(double x);
// extern double sin(double x);
// extern double sinh(double x);
// extern double tanh(double x);
// extern double exp(double x);
extern double frexp(double x, int* exponent);
extern double ldexp(double x, int exponent);
// extern double log(double x);
// extern double log10(double x);
extern double modf(double x, double *integer);
// extern double pow(double x, double y);
// extern double sqrt(double x);
// extern double ceil(double x);
// extern double fabs(double x);
// extern double floor(double x);
// extern double fmod(double x, double y);
 */