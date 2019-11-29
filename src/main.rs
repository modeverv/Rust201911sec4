fn main() {
    println!("Hello, world!");

    //let n = 42;
    //let c = 'R';

    let mut n = 0;
    println!("main:  n={}",n);
    f1(n);
    println!("main:  n={}",n);
    f2(&mut n);
    println!("main:  n={}",n);

    let mut f: fn(i32) -> i32  = double;
    assert_eq!(f(-42), -84);
    f = abs;
    assert_eq!(f(-22), 22);

    let v = vec!["I", "love", "Rust!"]
        .into_iter()
        .map(|s| s.len())
        .collect::<Vec<_>>();
    assert_eq!(v,vec![1,4,5]);

    {
        let ar = [1,2,3];
        for i in ar.iter() {
            println!("{}",i);
        }
        let mut ar2 = [1,2,3];
        for n in ar2.iter_mut() {
            *n *= 2;
        }
        for i in ar2.iter() {
            println!("{}",i);
        }
        
    }

    {
        let mut a4 = [1,2,3,4,0,-1,-2];
        let (a4a,a4b) = &mut a4.split_at_mut(3);
        for n in  (&a4a[..]).iter() {
            println!("{}",n);
        }
        for n in  (&a4b[..]).iter() {
            println!("{}",n);
        }
    }
    println!("-------\u{1f600}--------------------------");

}


fn f1(mut n:u32) {
    println!("f1:    n = {}",n);
    n = 1;
    println!("f1:    n = {}",n);
}
fn f2(n_ptr: &mut u32){
    println!("f2:  n_ptr={:p}",n_ptr);
    *n_ptr = 2;
    println!("f2:  *n_ptr={}",*n_ptr);
}

fn double(n: i32) -> i32{
    n + n
}
fn abs(n: i32) -> i32 {
    if n >= 0 { n } else { -n }
}