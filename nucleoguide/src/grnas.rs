use regex::Regex;

mod grnas {
    pub fn find(s: &String, p: &String, po: &usize) -> &'static str {
        todo!()
    }
}

fn re_pam_search(s: &String, p: &String) -> usize {
    let re = match Regex::new(p) {
        Ok(n) => n,
        Err(n) => panic!("Problem compiling pam into Regex: {:?}", n)
    };
    match re.find(s) {
        Some(n) => n.start(),
        None(n) => ()
    }
}