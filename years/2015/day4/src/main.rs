use md5::{Md5, Digest};

fn main() {
    const INPUT: &str = "bgvyzdsv";
    let mut count = 0;
    let mut hasher = Md5::new();

    loop {
        let test = format!("{}{}", INPUT, count);

        hasher.update(test.as_bytes());
        let result = hasher.finalize_reset();

        if result[0] == 0 && result[1] == 0 && result[2] < 16 {
            println!("result => {}", count);
            return ;
        } else {
            count += 1;
        }
    }
}
