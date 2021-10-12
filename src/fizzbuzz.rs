

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
