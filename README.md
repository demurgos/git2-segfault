# Git2 segfault repro

<https://github.com/rust-lang/git2-rs/issues/813>

```
$ uname -a
Linux red 5.16.12-arch1-1 #1 SMP PREEMPT Wed, 02 Mar 2022 12:22:51 +0000 x86_64 GNU/Linux
demurgos@red /data/projects/various/git2-segfault
$ pacman -Q | grep libgit2
libgit2 1:1.4.2-1
$ cargo +stable build
$ ldd ./target/debug/git2-segfault
        linux-vdso.so.1 (0x00007fffb4d39000)
        libssl.so.1.1 => /usr/lib/libssl.so.1.1 (0x00007fd97f8c7000)
        libcrypto.so.1.1 => /usr/lib/libcrypto.so.1.1 (0x00007fd97f5e6000)
        libz.so.1 => /usr/lib/libz.so.1 (0x00007fd97f5cc000)
        libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007fd97f5b1000)
        libc.so.6 => /usr/lib/libc.so.6 (0x00007fd97f3a7000)
        /lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x00007fd97fb2c000)
        libpthread.so.0 => /usr/lib/libpthread.so.0 (0x00007fd97f3a2000)
        libdl.so.2 => /usr/lib/libdl.so.2 (0x00007fd97f39b000)
$ cargo +stable run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/git2-segfault`
Segmentation fault (core dumped)
```

`strace.log` was generated with `strace ./target/debug/git2-segfault 2&> strace.log`.
