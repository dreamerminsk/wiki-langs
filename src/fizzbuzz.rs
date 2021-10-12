

struct Z;
struct S<T>(T);




trait Add<Rhs> {


    type Sum;


}

type SumOf<N, M>     =     <N as Add<M>>::Sum;
