fn main() {
    contact_me();
}

fn contact_me() {
    show_my_phone();
    show_my_email();
}

fn show_my_phone() {
    let p1 = "155";
    let p2 = "7545";
    let p3 = "0546";
    print!("{}-{}-{}\n", p1, p2, p3);
}

fn show_my_email() {
    let me = "binboy";
    print!("{}@live.com\n", me);
}