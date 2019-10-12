pub mod strmode {
    pub fn strmode(mode: u32) -> String {
        let mut flags = ['-'; 10];

        let perms = [
            (0o400, 'r'), (0o200, 'w'), (0o100, 'x'), // user
            (0o040, 'r'), (0o020, 'w'), (0o010, 'x'), // group
            (0o004, 'r'), (0o002, 'w'), (0o001, 'x'), // other
        ];

        // Permissions
        let s = &mut flags[1..];
        for i in 0..9 {
            if mode & perms[i].0 == perms[i].0 {
                s[i] = perms[i].1;
            }
        }

        // File type
        match mode & 0o170000 {
            0o010000    => { flags[0] = 'p' },  // fifo
            0o020000    => { flags[0] = 'c' },  // character special
            0o040000    => { flags[0] = 'd' },  // directory
            0o060000    => { flags[0] = 'b' },  // block special
            0o100000    => { },                 // regular file
            0o120000    => { flags[0] = 'l' },  // symbolic link
            0o140000    => { flags[0] = 's' },  // socket
            _           => { flags[0] = '?' },  // unknown
        }

        // TODO setuid, setgid, sticky

        return flags.into_iter().collect();
    }

    #[test]
    fn test_strmode() {
        let tests = [
            (0o100644, "-rw-r--r--"),
            (0o100600, "-rw-------"),
            (0o100777, "-rwxrwxrwx"),
            (0o040755, "drwxr-xr-x"),
            (0o040711, "drwx--x--x"),
            (0o020660, "crw-rw----"),
            (0o060660, "brw-rw----"),
            (0o120777, "lrwxrwxrwx"),
            (0o010600, "prw-------"),
            (0o140755, "srwxr-xr-x"),
        ];

        for t in &tests {
            assert_eq!(t.1, strmode(t.0));
        }
    }
}
