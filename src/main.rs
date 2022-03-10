use git2::{AutotagOption, FetchOptions, Repository, RepositoryInitOptions};
use std::path::PathBuf;

fn main() {
    let input_path = PathBuf::from("./git2");
    let remote_url = "https://gitlab.com/eternaltwin/popotamo/popotamo.git".to_string();
    let commit = "d48338d82864679ed3bc656d7f7d1c445ac991b2".to_string();

    std::fs::create_dir_all(input_path.as_path()).expect("create_dir");

    let mut init_opts = RepositoryInitOptions::new();
    init_opts.mkpath(false);
    init_opts.mkdir(false);
    init_opts.external_template(false);
    let repository = Repository::init_opts(&input_path, &init_opts).expect("init repp");

    let mut remote = repository.remote_anonymous(&remote_url).expect("create remote");
    {
        let mut fetch_opts = FetchOptions::new();
        fetch_opts.download_tags(AutotagOption::All);
        let all_refs: [&str; 3] = ["master", "develop", &commit];
        remote.fetch(&all_refs, Some(&mut fetch_opts), None).expect("fetch");
    }
    eprintln!("done");
}
