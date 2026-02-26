#![allow(
    unused_variables,
    dead_code,
    non_snake_case,
    unused_parens,
    unused_variables,
    non_snake_case,
    unreachable_code,
    unused_imports,
    unused_assignments,
    unused_mut
)]

use std::path::Component::ParentDir;
use std::collections::{HashMap,HashSet};
use itertools::Itertools;
use libm;
pub struct Problems;


impl Problems{
    pub fn problem_zero(&self,n:i64) -> i64{
        // Among the first 960 thousand square numbers, what is the sum of all the odd squares?
        let n = n / 2;
        let l : i64 = (2 * n) - 1;
        let r : i64 = (2 * n) + 1;
        let numerator = l * r * n;
        let sum : i64 =  numerator / 3;
        sum
    }
    pub fn problem_one(&self,a:i64,b:i64,n:i64) -> i64{
        // sum of all multiples of a or b < 1000 inclusion exclusion
        let n = n - 1;
        let M : f64 = libm::floor( n as f64 / a as f64);
        let M : i64 = M as i64;
        let N : f64 = libm::floor( n as f64 / b as f64);
        let N : i64 = N as i64;
        let NM : f64 = libm::floor( n as f64 / (a * b) as f64);
        let NM : i64 = NM as i64;
        let d : i64 = a * b;
        let numerator = ((a * M) * (M + 1)) + ((b * N) * (N + 1)) - ((NM * d) * (NM + 1));
        numerator / 2
    }
    pub fn problem_two(&self,N:i64) -> i64{
        let mut sum : i64 = 0;
        let mut n : i64 = 0;

        fn fib(n:i64) -> i64{
            if n == 0{
                return 0
            }else if n == 1{
                return 1
            }else{
                return fib(n-1) + fib(n-2)
            }
        }

        while fib(n) <= N{
            let f = fib(n);
            if f % 2 == 0{
                sum += f;
            }
            n += 1;
        }
        sum
    }
    pub fn problem_three(&self,N:i64) -> i64{
        // miller robin

        let mut largest : i64 = 2;
        let ceil : i64 = (N / 2) + 1;
        let mut prime_factors : Vec<i64> = vec![];

        fn prod(arr:&Vec<i64>) -> i64{
            let mut prod: i64 = 1;
            for i in 0..arr.len(){
                prod *= arr[i];
            }
            prod
        }

        fn is_prime(n:i64) -> bool{
            if n == 2 || n == 3{
                return true
            }
            let n_floor : i64 = (n/2) + 1;
            for i in 2..n_floor{
                if n % i == 0{
                    return false
                }
            }
            true
        }

        fn miller(n:i64,p:i64) -> i64{
            let mut c : i64 = 1;
            let mut n_ref : i64 = n / 2;
            while n_ref % p == 0{
                n_ref = n_ref / 2;
                c += 1;
            }
            p.pow(c as u32)
        }

        for d in 2..ceil{
            if N % d == 0 && is_prime(d) == true{
                if d > largest{
                    largest = d;
                    prime_factors.push(miller(N,d));
                }
                if prod(&prime_factors) == N{
                    return largest
                }
            }
        }
        largest
    }
    pub fn problem_four(&self,k:i64) -> i64{
        if k <= 1 {
            return -1
        }
        // min palindrome
        let t : i64 = 10;
        let mut min : i64  = 0;
        for i in 0..k{
            min += t.pow(k as u32)
        }
        min * min
    }
    pub fn problem_five(&self,N:i64) -> i64{



        fn gcd(a:i64,b:i64) -> i64{
            if b == 0{
                a
            }else{
                gcd(b,a%b)
            }
        }

        fn lcm(a:i64,b:i64) -> i64{
            let mut n : i64 = (a * b);
            n = n / gcd(a,b);
            n
        }

        // evenly divisible (a1,a2,a3,...N)

        // lcm(lcm(lcm(a1,a2),a3)...N)

        // lcm(a,b) = (a * b) / gcd(a,b)


        if N < 2{
            return -1
        }
        if N == 2{
            return lcm(1,2)
        }

        let mut start : i64 = lcm(1,2);


        for i in 3..N+1{
            start = lcm(start,i);
        }
        start


    }
    pub fn problem_six(&self,N:i64) -> i64{
        let mut sum_k2 : i64 = N * (N + 1) * (2*N + 1);
        sum_k2 /= 6;
        let mut sum_k : i64 = N * (N + 1);
        sum_k /= 2;
        sum_k = sum_k.pow(2);
        sum_k - sum_k2
    }
    pub fn problem_seven(&self,M:i64) -> i64{
        let mut start : i64 = 0;
        let mut n = 2;

        if M == 1{
            return 2
        }

        fn is_prime(n:i64) -> bool{
            if n == 2 || n == 3 || n == 5{
                return true
            }
            let n_floor : i64 = (n/2) + 1;
            for i in 2..n_floor{
                if n % i == 0{
                    return false
                }
            }
            true
        }

        while start != M{
            if is_prime(n) == true{
                start += 1;
            }
            n += 1;
        }
        n - 1
    }
    pub fn problem_eight(&self,arr:&Vec<i64>,k:usize) -> i64{
        // so since these have to be adjacent they are just sub arrays
        let n  = arr.len();

        fn prod(arr:&Vec<i64>) -> i64{
            let mut prod: i64 = 1;
            for i in 0..arr.len(){
                prod *= arr[i];
            }
            prod
        }
        
        let mut max_prod = 0;
        
        for i in 0..n{
            for j in i+1..n+1{
                if arr[i..j].len() == k{
                    if max_prod < prod(&arr[i..j].to_vec()){
                        max_prod = prod(&arr[i..j].to_vec());
                    }
                    
                }
            }
        }
        max_prod
    }
    pub fn problem_ten(&self,M:i64) -> i64{
        let mut p_sum : i64 = 0;
        let mut n = 2;

        if M == 1{
            return 2
        }

        fn is_prime(n:i64) -> bool{
            if n == 2 || n == 3 || n == 5{
                return true
            }
            let n_floor : i64 = (n/2) + 1;
            for i in 2..n_floor{
                if n % i == 0{
                    return false
                }
            }
            true
        }

        while n < M{
            if is_prime(n) == true{
                p_sum += n;
                println!(" nth prime {:?}",n);
            }
            n += 1;
        }
        p_sum
    }
    pub fn problem_eleven(&self,arr:&Vec<Vec<i64>>,k:usize) -> i64{
        if k <= 0 {
            return 0
        }

        if arr.is_empty() == true {
            return 0
        }

        let n = arr[0].len();

        let mut arr_ref = arr.clone();

        let mut max_p : i64 = 0;

        for i in 0..n{
            for j in 0..n{
                arr_ref[i][j] = arr[j][i];
            }
        }


        fn brute_force_max_product(arr:&Vec<i64>,k:usize) -> i64{
            let mut max_p : i64 = 0;

            for i in 0..=arr.len() - k{
                let subarray_product : i64 = arr[i..i+k].iter().product();
                if subarray_product > max_p{
                    max_p = subarray_product;
                }
            }
            max_p
        }




        fn collect_diagonals(arr: &Vec<Vec<i64>>, k: usize) -> Vec<Vec<i64>> {
            let m = arr.len();
            let mut all_diagonals: Vec<Vec<i64>> = vec![];

            // Top-left → bottom-right diagonals
            for start_row in 0..m {
                let mut diag: Vec<i64> = vec![];
                let mut r = start_row;
                let mut c = 0;
                while r < m && c < m {
                    diag.push(arr[r][c]);
                    r += 1;
                    c += 1;
                }
                if diag.len() >= k {
                    all_diagonals.push(diag);
                }
            }
            for start_col in 1..m {
                let mut diag: Vec<i64> = vec![];
                let mut r = 0;
                let mut c = start_col;
                while r < m && c < m {
                    diag.push(arr[r][c]);
                    r += 1;
                    c += 1;
                }
                if diag.len() >= k {
                    all_diagonals.push(diag);
                }
            }

            // Top-right → bottom-left diagonals
            for start_col in 0..m {
                let mut diag: Vec<i64> = vec![];
                let mut r = 0;
                let mut c = start_col;
                while r < m && c < m {
                    diag.push(arr[r][c]);
                    r += 1;
                    if c == 0 { break; } // prevent underflow
                    c -= 1;
                }
                if diag.len() >= k {
                    all_diagonals.push(diag);
                }
            }
            for start_row in 1..m {
                let mut diag: Vec<i64> = vec![];
                let mut r = start_row;
                let mut c = m - 1;
                while r < m && c < m { // c < m always true because c = usize
                    diag.push(arr[r][c]);
                    r += 1;
                    if c == 0 { break; }
                    c -= 1;
                }
                if diag.len() >= k {
                    all_diagonals.push(diag);
                }
            }

            all_diagonals
        }



        let arr_k : Vec<Vec<i64>> = collect_diagonals(arr,k);

        let arr_ref_k : Vec<Vec<i64>> = collect_diagonals(&arr_ref,k);


        for i in 0..arr_k.len(){
            let lp = brute_force_max_product(&arr_k[i],k);
            if lp > max_p{
                max_p = lp;
            }
        }

        for i in 0..arr_ref_k.len(){
            let lp = brute_force_max_product(&arr_ref_k[i],k);
            if lp > max_p{
                max_p = lp;
            }
        }



        max_p


    }
    pub fn problem_thirteen(&self,arr:&Vec<i64>) -> i64{
        arr.iter().sum()
    }
    pub fn problem_fourteen(&self,N:i128,s:i128) -> i128{
        
        let mut max_length : i128 = 0;
        let mut max_start : i128 =  s;
        
        
        fn collatz(n:i128) -> i128{
            if n % 2 == 0{
                (n / 2)
            }else{
                (3 * n) + 1
            }
        }
        
        fn collatz_sequence(n:i128) -> i128{
            if n == 1{
                1
            }else{
                let mut m : i128 = n;
                let mut sequence_length : i128 = 1;
                while m != 1{
                    m = collatz(m);
                    sequence_length += 1;
                }
                sequence_length
            }
        }
        
        
        for i in s..N+1{
            let sequence_length = collatz_sequence(i);
            if sequence_length > max_length{
                max_length = sequence_length;
                max_start = i;
            }
            
        }
        max_start
    }
    pub fn problem_fifteen(&self,n:i128) -> i128{
        if n > 16{
            return -1
        }
        fn factorial(q:i128) -> i128 {
            if q == 0 {
                1
            } else {
                q * factorial(q - 1)
            }
        }
        
        
        fn choose(c:i128,r:i128) -> i128{
            let C : i128 = 2 * c;
            factorial(C) / (factorial(r) * factorial(C - r))
        }
        
        
        choose(n,n)
        
        
        
        
        
        
        
        
        
    }
    pub fn problem_twenty_one(&self,n:i128) -> i128{

        let mut seen : HashSet<i128> = HashSet::new();

        fn proper_d(n:i128) -> Vec<i128>{
            let mut a_sum : i128 = 1;
            let mut b_sum : i128 = 1;

            let n_limit : i128 = (n/2) + 1;

            for num in 2..n_limit{
                if n % num == 0{
                    a_sum += num;
                }
            }

            let a_limit : i128 = (a_sum/2) + 1;

            for num in 2..(a_limit){
                if a_sum % num == 0{
                    b_sum += num;
                }
            }

            if n != a_sum && b_sum == n {
                vec![n,a_sum]
            }else{
                vec![0,0]
            }









        }




        for number in 2..n{
            seen.insert(proper_d(number)[0]);
            seen.insert(proper_d(number)[1]);
        }

        println!(" set {:?}",seen);

        seen.iter().sum()











    }
    pub fn problem_twenty_three(&self,n:i128) -> i32 {
        if n > 1000{
            return -1
        }
        fn perfect(n: i128) -> bool {
            let d_limit = (n / 2) + 1;
            let mut d_sum: i128 = 0;


            for d in 1..d_limit {
                if n % d == 0 {
                    d_sum += d;
                }
            }
            d_sum == n
        }

        fn abundant(n: i128) -> bool {
            let d_limit = (n / 2) + 1;
            let mut d_sum: i128 = 0;

            for d in 1..d_limit {
                if n % d == 0 {
                    d_sum += d;
                }
            }
            d_sum > n
        }

        fn deficient(n: i128) -> bool {
            let d_limit = (n / 2) + 1;
            let mut d_sum: i128 = 0;


            for d in 1..d_limit {
                if n % d == 0 {
                    d_sum += d;
                }
            }
            if d_sum < n{
                true
            }else{
                false
            }
        }



        let mut a_nums : Vec<i128> = vec![];



        for num in 1..n{
            if abundant(num) == true{
                a_nums.push(num);
            }
        }

        let mccp : Vec<Vec<i128>> = std::iter::repeat(a_nums.iter()).take(2).multi_cartesian_product().into_iter()
            .map(|xs| xs.into_iter().copied().collect())
            .collect();
        let mut c : i32 = 0;
        let mut na : i32 = 0;
        for i in 25..n{
            println!(" i {:?}",i);
            for p in mccp.iter(){
                let s : i128 = p.iter().sum();
                if s == i{
                    break
                }
                c += 1;
            }
            if c == mccp.len() as i32{
                na += c;
            }
            c = 0;
        }
        na
        

    }
    pub fn p31(&self,c:Vec<i32>,T:usize) -> i32{
        if (T == 0) && (c.is_empty() == true){
            return 1
        }
        if (T > 0) && (c.is_empty() == true){
            return 0
        }
        // then we make the DP table , dim(DP) = (n + 1) x * (T + 1) , n = c.len() + 1
        let N : usize = c.len() + 1;
        
        let mut DP : Vec<Vec<i32>> = vec![vec![0;N];T];
        
        // then we hardcode the base case DP[0][0] = 1
        
        DP[0][0] = 1;
        
        // use recurrence relation DP[i][j] = DP[i - 1][j] + DP[i][j - c_i] <=> (j >= c_i , otherwise DP[i][j] = 0)
        
        // i = 0....(T - 1), where we have (N - 1) coins
        // j = 0....(N - 1) , and we are looking for target T
        for i in 1..N{
            for j in 0..(T + 1){
                if ((j as i32) >= c[i]){
                    DP[i][j] = DP[i - 1][j] + DP[i][j - (c[i] as usize)];
                }else{
                    DP[i][j] = 0;
                }
            }
        }
        DP[N][T] // is the total number of ways using (N - 1) denomination coins to sum up to target (T - 1)
        
    }
    pub fn p33(&self,n:i32) -> i32{
        
        
        fn gcd(a:i32,b:i32) -> i32{
            if b == 0{
                a
            }else{
                gcd(b,a%b)
            }
        }
        
        fn lcm(a:i32,b:i32) ->i32{
            ((a * b) / gcd(a , b))
        }
        

        
        
        let mut num : i32 = 1;
        let mut den : i32 = 1;
        
        
        
        for i in 11..100{ // possible fractions ((99 - 11) + 1)^2 => (7744 - 8 * 1 fractions) fractions
            for j in 11..100{
                
                if i == j{
                    continue
                }
                
                if lcm(j,i) == gcd(j,i) && ((i % 10 != 0) && (j % 10 != 0)) && (gcd(j,i) != 1){
                    num *= i;
                    den *= j;
                }
            }
        }
        
        let d : i32 = gcd(den,num);
        
        num
        
        
    }
    pub fn p34(&self,base:i64,upper_limit:i64) -> i64{
        // let m be the upper limit
        
        fn fac(n:i64)-> i64{
            if n == 0{
                1
            }else{
                let m : i64 = n - 1;
                n * fac(m)
            }
        }
        
        fn digits_factorial_sum(arr:&Vec<i64>) -> i64{
            let s : i64 = arr.iter().map(|x| fac(*x)).sum();
            s
        }
        
        let mut sum : i64 = 0;
        
        
        fn strip_sum(m:i64,n:i64) -> i64{
            let mut d : Vec<i64> = vec![];
            let mut m_ref = m;
            while m_ref > 0{
                let digit : i64 = (m_ref % n);
                d.push(digit);
                m_ref = m_ref / n;
            }
            digits_factorial_sum(&d)
        }
        
        
        
        for num in 3..upper_limit{
            if strip_sum(num,base) == num{
                sum += num;
                
            }
        }
        sum
    }
    pub fn p35(&self,m:i64) -> usize{
        let mut total_primes : Vec<i64> = vec![];


        fn is_prime(n:i64) -> bool{
            if n == 2 || n == 3 || n == 5 || n == 7 {
                return true
            }
            let n_floor : i64 = (n/2) + 1;
            for i in 2..n_floor{
                if n % i == 0{
                    return false
                }
            }
            true
        }
        
        
        fn strip_sum(m:i64,n:i64) -> Vec<i64>{
            let mut d : Vec<i64> = vec![];
            let mut m_ref = m;
            while m_ref > 0{
                let digit : i64 = (m_ref % n);
                d.push(digit);
                m_ref = m_ref / n;
            }
            d
        }
        
        fn get_rotations(arr:Vec<i64>) -> bool{
            let mut rotations : Vec<Vec<i64>> = vec![];
            let mut rotations_nums : Vec<i64> = vec![];
            
            let mut arr_ref : Vec<i64> = arr;

            let mut m = arr_ref.len();
            let mut m_ref = m;
            let e_index  = m - 1;

            while m_ref > 0{
                let mut uni = vec![arr_ref[m - 1]];
                let end = arr_ref[0..e_index].to_vec();
                uni.extend(end);
                let mut rotated = uni;
                rotations.push(rotated.clone());
                arr_ref = rotated;
                m_ref -= 1;
            }
            
            for ro in rotations{
                let mut s: i64 = 0;
                let t : i64 = 10;
                for d in (0..ro.len()).rev(){
                    s += (ro[d] * (t.pow(d as u32)));
                }
                rotations_nums.push(s);
                s = 0;
            }
            
            
            for v in rotations_nums{
                if is_prime(v) == false{
                    return false
                }
            }
            true
        }
        
        
        for number in 2..m{
            println!("Number {:?}",number);
            if is_prime(number) == true{
                let arr : Vec<i64> = strip_sum(number,10);
                if get_rotations(arr) == true{
                    total_primes.push(number);
                }
            }else{
                continue
            }
        }
        
        total_primes.len()
    }
    
    
    
    
    
    
            
    
    
    
    
    
    

}



