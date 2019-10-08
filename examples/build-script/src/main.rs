fn main() {
    println!("git_commit_id: {}", env!("DIRT_GIT_COMMIT_ID"));
    println!("git_commit_short_id: {}", env!("DIRT_GIT_COMMIT_SHORT_ID"));
    println!("git_status: {}", env!("DIRT_GIT_REPOSITORY_STATUS"));
}
