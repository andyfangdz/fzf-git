extern crate git2;
extern crate quicli;
extern crate hex;
#[macro_use]
extern crate itertools;
extern crate chrono;
extern crate tabwriter;

use quicli::prelude::*;
use git2::Repository;
use std::env;
use chrono::NaiveDateTime;
use tabwriter::TabWriter;
use std::io::Write as IoWrite;

fn main() -> CliResult {
    let repo = Repository::discover(env::current_dir()?)?;
    let branches: Vec<_> = repo.branches(Some(git2::BranchType::Local))?
        .map(|branch| String::from(branch.unwrap().0.name().unwrap().unwrap()))
        .collect();

    let mut branch_oids: Vec<_> = branches.clone().into_iter()
        .map(|branch| repo.revparse_single(branch.as_str()).unwrap().id())
        .collect();

    let commits: Vec<_> = branch_oids.clone().into_iter()
        .map(|oid| repo.find_commit(oid).unwrap())
        .collect();

    let commit_times: Vec<_> = commits.clone().into_iter()
        .map(|commit| NaiveDateTime::from_timestamp(commit.time().seconds(), 0))
        .collect();

    let commit_shas: Vec<String> = branch_oids.clone().into_iter()
        .map(|oid| hex::encode(oid.as_bytes()).chars().take(6).collect())
        .collect();

    let commit_msgs: Vec<String> = commits.clone().into_iter()
        .map(|commit| {
            let msg: String = commit.message().unwrap().chars().take(20).collect();
            msg.trim_end().to_owned()
        })
        .collect();
    let mut tw = TabWriter::new(vec![]);
    izip!(branches.into_iter(), commit_shas.into_iter(), commit_times.into_iter(), commit_msgs.into_iter())
        .for_each(|(a, b, c, d)| write!(&mut tw, "{}\t{}\t{}\t{}\n", a, b, c, d).unwrap());
    print!("{}", String::from_utf8(tw.into_inner().unwrap()).unwrap());
    Ok(())
}
