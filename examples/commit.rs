extern crate dirt_build;

fn main() {
    let repo = dirt_build::Repository::new(None).expect("Failed to init repository");

    println!("commit id {:?}", repo.commit_id().expect("fail"));
    println!("commit short id {:?}", repo.commit_short_id().expect("fail"));
    println!("repo status {:?}", repo.status().expect("fail"));
}
