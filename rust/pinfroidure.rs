use std::ops::Mul;

// Store transformations as vectors for the time being

trait CanIndexVec {
    fn as_usize(&self) -> usize;
}

impl CanIndexVec for u8 {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

impl CanIndexVec for u16 {
    fn as_usize(&self) -> usize {
        *sef as usize
    }
}

#[derive(Eq, Debug)]
struct Trans<T>(Vec<T>);

impl<T: CanIndexVec + Clone> Mul for Trans<T> {
    type Output = Trans<T>;

    fn mul(self, _rhs: Trans<T>) -> Trans<T> {
        let Trans(l) = self;
        let Trans(r) = _rhs;
        
        Trans::<T>(l.iter().map(|i| r[i.as_usize()].clone()).collect())
    }
}
/*
fn at<T>(t: Trans<T>,i:i32) -> T
{
    let Trans::<T>(tr) = t;
    
    return tr[i];
}
*/

fn pin_froidure<S: Mul + Eq>(gens: Vec<S>) -> Semigroup<S>
{
}


fn main()
{
    let a = Trans::<u8>(vec![0,1,3,1]);
    let b = Trans::<u8>(vec![2,1,0,3]); 
    println!("Vector: {:?}", a * b);
    println!("Hello, world\n");
}
