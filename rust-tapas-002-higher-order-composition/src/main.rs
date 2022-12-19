
// we want to rturn a function when called
fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x: A| g(f(x))
}

fn partial<A, B, C, F, G>(f: F) -> impl Fn(G) -> Box<dyn Fn(A) -> C>
where
    F: Fn(A) -> B + 'static + Copy ,
    G: Fn(B) -> C + 'static ,
    // H: fn(A) -> C ,
{
    move |g: G| Box::new(move |a: A| g(f(a)))
}

// macro_rules! curry{
//     (
//         $f:ident
//         $(
//             ($args:ident : $types:ty)
//         )*
//     ) => {
//     $(move |$args: $types|)*
//     $f(
//       $($args),*
//     )
//   }
// }

macro_rules! curry (
   // Simplest form, without any type annotations.
    (|$first_arg:ident $(, $arg:ident )*| $function_body:expr) => {
       move |$first_arg| $(move |$arg|)* {
          $function_body
       }
    };
    // With input type annotations
    (|$first_arg:ident:$first_arg_type:ty $(, $arg:ident:$arg_type:ty )*| $function_body:expr) => {
      move |$first_arg:$first_arg_type| $(move |$arg:$arg_type|)* {
         $function_body
      }
   };
   // With input and return type annotations and a block as function body
   (|$first_arg:ident:$first_arg_type:ty $(, $arg:ident:$arg_type:ty )*| -> $ret_type:ty $function_body:block) => {
    move |$first_arg:$first_arg_type| $(move |$arg:$arg_type|)* -> $ret_type {
       $function_body
    };
   };
);

fn main() {
    let double = |x| 2 * x;

    let f  = move |a| move |b| a + b;
    let sum = f(1)(2);
    dbg!(sum);

    let cf= curry!(|a: u32,b: u32,c:u32| double(a) + b + c);

    let f = cf(1)(2)(1);
    dbg!(f);



}
