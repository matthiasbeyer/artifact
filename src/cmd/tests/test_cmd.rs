
use super::super::types::*;
use super::super::matches::*;
use super::super::ls;
use super::super::check;

use std::thread;
use std::time;


#[test]
/// partof: #TST-ls-interface
fn test_get_matches() {
    let args = vec!["rst", "ls", "-l"];
    let matches = get_matches(&args).unwrap();
    let (search, fmtset, search_set) = ls::get_ls_cmd(matches.subcommand_matches("ls").unwrap())
                                           .unwrap();
    assert_eq!(search, "");
    assert_eq!(fmtset.long, true);
    assert_eq!(fmtset.recurse, 0);
    assert_eq!(search_set, SearchSettings::new());

    // test that -A works
    let args = vec!["rst", "ls", "all", "-AP"];
    let matches = get_matches(&args).unwrap();
    let (search, fmtset, search_set) = ls::get_ls_cmd(matches.subcommand_matches("ls").unwrap())
                                           .unwrap();
    assert_eq!(search, "all");
    assert_eq!(fmtset.long, false);
    assert_eq!(fmtset.parts, false);
    assert_eq!(fmtset.partof, true);
    assert_eq!(fmtset.loc_path, true);
    assert_eq!(fmtset.recurse, 0);
    assert_eq!(search_set, SearchSettings::new());

    // test that pattern works
    // #TST-ls-search
    let args = vec!["rst", "ls", "regex", "-p", "TNL"];
    let matches = get_matches(&args).unwrap();
    let (search, _, search_set) = ls::get_ls_cmd(matches.subcommand_matches("ls").unwrap())
                                      .unwrap();
    assert_eq!(search, "regex");
    assert!(search_set.text);
    assert!(search_set.name);
    assert!(search_set.loc);
}


#[test]
fn test_cmds() {
    let (mut fmt_set, mut search_set, mut settings) = (FmtSettings::default(),
                                                       SearchSettings::default(),
                                                       Settings::default());
    let mut artifacts = core::load::load_toml_simple(r"
[REQ-foo]
text = 'req for foo'
[SPC-foo]
text = 'spc for foo'
[TST-foo]
text = 'tst for foo'
[TST-foo_bar]
partof = 'SPC-foo'
text = 'tst for foo_bar'

[SPC-unresolvable]
partof = 'SPC-unresolvable-1-1'
[SPC-unresolvable-1]
[SPC-unresolvable-1-1]

[REQ-invalid-parts]
partof = 'REQ-dne'
");
    let reqs_path = PathBuf::from("reqs/foo.toml");
    for (n, a) in artifacts.iter_mut() {
        a.path = reqs_path.clone();
        if n.as_ref() == &ArtName::from_str("spc-foo").unwrap() {
            a.loc = Some(Loc::fake());
        }
        if n.as_ref() == &ArtName::from_str("tst-foo").unwrap() {
            a.loc = Some(Loc::fake());
        }
    }
    core::link::do_links(&mut artifacts).unwrap();
    fmt_set.color = true;
    settings.color = true;
    let mut w: Vec<u8> = Vec::new();
    let cwd = PathBuf::from("src/foo");


    // define helper functions
    fn vb(b: &'static [u8]) -> Vec<u8> {
        Vec::from_iter(b.iter().cloned())
    }
    /// if the format changes, you can use this to help create the test for color
    /// just pass it in and copy-paste (validating that it looks right first of course...)
    #[allow(dead_code)]
    fn debug_bytes(bytes: &Vec<u8>) {
        // sleep for a bit so stderr passes us
        thread::sleep(time::Duration::new(0, 2e8 as u32));
        println!("Debug:");
        for b in bytes {
            print!("{}", *b as char);
        }
        for b in bytes {
            match *b {
                // 9 => print!("{}", *b as char), // TAB
                b'\n' => print!("\\n"),
                b'\r' => print!("\\r"),
                32...126 => print!("{}", *b as char), // visible ASCII
                _ => print!(r"\x{:0>2x}", b),

            }
        }
        println!("");
    }

    let dne_locs: HashMap<_, _> = HashMap::from_iter(vec![(ArtName::from_str("SPC-dne").unwrap(),
                                                           Loc::fake())]);

    // #TST-check
    w.clear();
    check::do_check(&mut w, &cwd, &artifacts, &dne_locs, &settings);
    let expected = b"\x1b[1;31m\nFound partof names that do not exist:\n\x1b[0m\x1b[31m    REQ-invalid-parts [../../reqs/foo.toml]: {REQ-DNE}\n\x1b[0m\x1b[1;31m\nArtifacts partof contains at least one recursive reference:\n\x1b[0m    SPC-unresolvable              : [SPC-UNRESOLVABLE-1-1]\n    SPC-unresolvable-1            : [SPC-UNRESOLVABLE]\n    SPC-unresolvable-1-1          : [SPC-UNRESOLVABLE-1]\n\x1b[1;31m\nFound implementation links in the code that do not exist:\n\x1b[0m\x1b[31m    ../../fake:\n\x1b[0m\x1b[31m    - (42:0)\x1b[0m SPC-dne\n\x1b[1;31m\nHanging artifacts found (top-level but not partof a higher type):\n\x1b[0m    ../../reqs/foo.toml           : SPC-unresolvable\n\n";
    assert_eq!(vb(expected), w);

    // #TST-ls
    // do default list, looking for only req-foo
    w.clear();
    ls::do_ls(&mut w,
              &cwd,
              "req-foo",
              &artifacts,
              &fmt_set,
              &search_set,
              &settings);
    let expected = b"\x1b[1m|  | DONE TEST | ARTIFACT NAME                                 | PARTS   | DEFINED   \n\x1b[0m|\x1b[1;34mD\x1b[0m\x1b[1;33m-\x1b[0m| \x1b[1;34m100\x1b[0m%  \x1b[1;33m50\x1b[0m% | \x1b[1;4;34mreq-foo\x1b[0m                                       | \x1b[34mSPC-foo\x1b[0m | ../../reqs/foo.toml \n";
    assert_eq!(vb(expected), w);

    // do default list with color disabled
    w.clear();
    fmt_set.color = false;
    ls::do_ls(&mut w,
              &cwd,
              "req-foo",
              &artifacts,
              &fmt_set,
              &search_set,
              &settings);
    let expected = b"|  | DONE TEST | ARTIFACT NAME                                 | PARTS   | DEFINED   \n|D-| 100%  50% | req-foo                                       | SPC-foo | ../../reqs/foo.toml \n";
    assert_eq!(vb(expected), w);

    // ls all fields
    // do a search in only parts using regex s.c
    w.clear();
    fmt_set.color = true;
    fmt_set.path = true;
    fmt_set.parts = true;
    fmt_set.partof = true;
    fmt_set.loc_path = true;
    fmt_set.text = true;
    search_set.use_regex = true;
    search_set.parts = true;
    ls::do_ls(&mut w,
              &cwd,
              "s.c.*foo",
              &artifacts,
              &fmt_set,
              &search_set,
              &settings);
    let expected = b"\x1b[1m|  | DONE TEST | ARTIFACT NAME                                 | PARTS   | PARTOF   | IMPLEMENTED   | DEFINED   | TEXT\n\x1b[0m|\x1b[1;34mD\x1b[0m\x1b[1;33m-\x1b[0m| \x1b[1;34m100\x1b[0m%  \x1b[1;33m50\x1b[0m% | \x1b[1;4;34mREQ-foo\x1b[0m                                       | \x1b[34mSPC-foo\x1b[0m | \x1b[33mREQ\x1b[0m | ../../reqs/foo.toml | req for foo \n|\x1b[1;5;31m!\x1b[0m\x1b[1;5;31m!\x1b[0m|  \x1b[1;5;31m-1\x1b[0m%  \x1b[1;5;31m-1\x1b[0m% | \x1b[1;4;31mSPC\x1b[0m                                           | \x1b[34mSPC-foo\x1b[0m, \x1b[31mSPC-unresolvable\x1b[0m |  | PARENT | AUTO \n";
    assert_eq!(vb(expected), w);
}
