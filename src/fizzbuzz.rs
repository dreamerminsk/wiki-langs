

struct Z;
struct S<T>(T);




trait Add<Rhs> {


    type Sum;


}

type SumOf<N, M>     =     <N as Add<M>>::Sum;


impl<N> Add<N> for Z {
    type Sum = N;
}

impl<N, M> Add<M> for S<N>
where
    N: Add<S<M>>,
{
    type Sum = SumOf<N, S<M>>;
}



type One = S<Z>;
type Two = SumOf<One, One>;
type Three = SumOf<One, Two>;
type Five = SumOf<Two, Three>;
type Ten = SumOf<Five, Five>;
type TwentyFive = SumOf<Five, SumOf<Ten, Ten>>;
type Fifty = SumOf<TwentyFive, TwentyFive>;
type OneHundred = SumOf<Fifty, Fifty>;

type N = OneHundred;


struct Nil;
struct Cons<H, T>(H, T);
