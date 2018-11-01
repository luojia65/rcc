macro_rules! f64_defs {
    ($($name: ident )+) => {
        $(
            #[no_mangle]
            pub extern "C" fn $name(x: f64) -> f64 {
                f64::$name(x)
            }
        )+
    };
}

f64_defs! {
    acos asin atan cos cosh sin sinh tanh
    exp log10 sqrt ceil floor
}

#[no_mangle]
pub extern "C" fn atan2(x: f64, y: f64) -> f64 {
    f64::atan2(x, y)
}

#[no_mangle]
pub extern "C" fn pow(x: f64, y: f64) -> f64 {
    x.powf(y)
}

#[no_mangle]
pub extern "C" fn fabs(x: f64) -> f64 {
    x.abs()
}

#[no_mangle]
pub extern "C" fn log(x: f64) -> f64 {
    x.ln()
}


#[no_mangle]
pub extern "C" fn fmod(x: f64, y: f64) -> f64 {
    x.mod_euc(y)
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