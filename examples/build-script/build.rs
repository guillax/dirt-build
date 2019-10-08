extern crate dirt_build;

fn main() {
  let repo = dirt_build::Repository::new(None).expect("Failed to initialize repository");

  println!("cargo:rustc-env=DIRT_GIT_COMMIT_ID={}", repo.commit_id().expect("Failed to get commit id"));
  println!("cargo:rustc-env=DIRT_GIT_COMMIT_SHORT_ID={}", repo.commit_short_id().expect("Failed to get commit short id"));
  println!("cargo:rustc-env=DIRT_GIT_REPOSITORY_STATUS={}", repo.status().expect("Failed to get repository status"));
}