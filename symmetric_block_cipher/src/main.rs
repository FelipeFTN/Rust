fn main() {
    println!("Hello, world!");
}

pub trait Word:
Clone + Copy +
std::ops::AddAssign +
std::ops::BitXor<Output = Self> +
std::ops::Add<Output = Self> +
std::ops::Shl<Output = Self> +
std::ops::Sub<Output = Self>
{
    const ZERO: Self;
}

// Encryption
// A = A + S[0]
// B = B + S[1] 
// for i = 1 to 2 * (r + 1) do
//    A = ((A xor B) << B) + S[2 * i]
//    B = ((B xor A) << A) + S[2 * i + 1]
// end

pub fn encrypt<W: Word>(plaintext: [W; 2], rounds: usize) -> [W; 2] {
    let t: usize = 2 * (rounds + 1);
    let s: Vec<W> = vec![W::ZERO; t];

    let [mut a, mut b] = plaintext;

    a += s[0];
    b += s[1];

    for i in 0..t {
        a = ((a ^ b) << b) + s[2 * i];
        b = ((b ^ a) << a) + s[2 * i + 1];
    }
    [a, b]
}

// Decryption
// for i = 2 * r + 1 to 1 do
//      B = (B - S[2 * i + 1]) >> A) xor A
//      A = (A - S[2 * i]) >> B) xor B
// B = B - S[1]
// A = A - S[0]

pub fn decrypt<W: Word>(ciphertext: [W; 2], rounds: usize) -> [W; 2] {
    let t: usize = 2 * (rounds + 1);
    let s: Vec<W> = vec![W::ZERO; t];

    let [mut a, mut b] = ciphertext;

    a += s[0];
    b += s[1];

    for i in 0..t {
        a = ((a ^ b) << b) + s[2 * i];
        b = ((b ^ a) << a) + s[2 * i + 1];
    }
    a += s[0];
    b += s[1];
    [a - s[0], b - s[1]]
}

