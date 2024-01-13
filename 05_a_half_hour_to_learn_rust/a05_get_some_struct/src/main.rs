struct SomeStruct {
    nickname: String,
}

fn get_some_struct() -> SomeStruct {
    SomeStruct {
        nickname: "fasterthanlime".to_string(),
    }
}
fn main() {
    let nick = "fasterthanlime";
    dbg!(nick.len());
    let amos = get_some_struct();
    dbg!(amos.nickname);
}
