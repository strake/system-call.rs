// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! System call numbers for x86-64 FreeBSD.

pub const SYSCALL: usize = 0;
pub const EXIT: usize = 1;
pub const FORK: usize = 2;
pub const READ: usize = 3;
pub const WRITE: usize = 4;
pub const OPEN: usize = 5;
pub const CLOSE: usize = 6;
pub const WAIT4: usize = 7;
pub const LINK: usize = 9;
pub const UNLINK: usize = 10;
pub const CHDIR: usize = 12;
pub const FCHDIR: usize = 13;
pub const MKNOD: usize = 14;
pub const CHMOD: usize = 15;
pub const CHOWN: usize = 16;
pub const BREAK: usize = 17;
pub const FREEBSD4_GETFSSTAT: usize = 18;
pub const GETPID: usize = 20;
pub const MOUNT: usize = 21;
pub const UNMOUNT: usize = 22;
pub const SETUID: usize = 23;
pub const GETUID: usize = 24;
pub const GETEUID: usize = 25;
pub const PTRACE: usize = 26;
pub const RECVMSG: usize = 27;
pub const SENDMSG: usize = 28;
pub const RECVFROM: usize = 29;
pub const ACCEPT: usize = 30;
pub const GETPEERNAME: usize = 31;
pub const GETSOCKNAME: usize = 32;
pub const ACCESS: usize = 33;
pub const CHFLAGS: usize = 34;
pub const FCHFLAGS: usize = 35;
pub const SYNC: usize = 36;
pub const KILL: usize = 37;
pub const GETPPID: usize = 39;
pub const DUP: usize = 41;
pub const PIPE: usize = 42;
pub const GETEGID: usize = 43;
pub const PROFIL: usize = 44;
pub const KTRACE: usize = 45;
pub const GETGID: usize = 47;
pub const GETLOGIN: usize = 49;
pub const SETLOGIN: usize = 50;
pub const ACCT: usize = 51;
pub const SIGALTSTACK: usize = 53;
pub const IOCTL: usize = 54;
pub const REBOOT: usize = 55;
pub const REVOKE: usize = 56;
pub const SYMLINK: usize = 57;
pub const READLINK: usize = 58;
pub const EXECVE: usize = 59;
pub const UMASK: usize = 60;
pub const CHROOT: usize = 61;
pub const MSYNC: usize = 65;
pub const VFORK: usize = 66;
pub const SBRK: usize = 69;
pub const SSTK: usize = 70;
pub const VADVISE: usize = 72;
pub const MUNMAP: usize = 73;
pub const MPROTECT: usize = 74;
pub const MADVISE: usize = 75;
pub const MINCORE: usize = 78;
pub const GETGROUPS: usize = 79;
pub const SETGROUPS: usize = 80;
pub const GETPGRP: usize = 81;
pub const SETPGID: usize = 82;
pub const SETITIMER: usize = 83;
pub const SWAPON: usize = 85;
pub const GETITIMER: usize = 86;
pub const GETDTABLESIZE: usize = 89;
pub const DUP2: usize = 90;
pub const FCNTL: usize = 92;
pub const SELECT: usize = 93;
pub const FSYNC: usize = 95;
pub const SETPRIORITY: usize = 96;
pub const SOCKET: usize = 97;
pub const CONNECT: usize = 98;
pub const GETPRIORITY: usize = 100;
pub const BIND: usize = 104;
pub const SETSOCKOPT: usize = 105;
pub const LISTEN: usize = 106;
pub const GETTIMEOFDAY: usize = 116;
pub const GETRUSAGE: usize = 117;
pub const GETSOCKOPT: usize = 118;
pub const READV: usize = 120;
pub const WRITEV: usize = 121;
pub const SETTIMEOFDAY: usize = 122;
pub const FCHOWN: usize = 123;
pub const FCHMOD: usize = 124;
pub const SETREUID: usize = 126;
pub const SETREGID: usize = 127;
pub const RENAME: usize = 128;
pub const FLOCK: usize = 131;
pub const MKFIFO: usize = 132;
pub const SENDTO: usize = 133;
pub const SHUTDOWN: usize = 134;
pub const SOCKETPAIR: usize = 135;
pub const MKDIR: usize = 136;
pub const RMDIR: usize = 137;
pub const UTIMES: usize = 138;
pub const ADJTIME: usize = 140;
pub const SETSID: usize = 147;
pub const QUOTACTL: usize = 148;
pub const NLM_SYSCALL: usize = 154;
pub const NFSSVC: usize = 155;
pub const FREEBSD4_STATFS: usize = 157;
pub const FREEBSD4_FSTATFS: usize = 158;
pub const LGETFH: usize = 160;
pub const GETFH: usize = 161;
pub const FREEBSD4_GETDOMAINNAME: usize = 162;
pub const FREEBSD4_SETDOMAINNAME: usize = 163;
pub const FREEBSD4_UNAME: usize = 164;
pub const SYSARCH: usize = 165;
pub const RTPRIO: usize = 166;
pub const SEMSYS: usize = 169;
pub const MSGSYS: usize = 170;
pub const SHMSYS: usize = 171;
pub const FREEBSD6_PREAD: usize = 173;
pub const FREEBSD6_PWRITE: usize = 174;
pub const SETFIB: usize = 175;
pub const NTP_ADJTIME: usize = 176;
pub const SETGID: usize = 181;
pub const SETEGID: usize = 182;
pub const SETEUID: usize = 183;
pub const STAT: usize = 188;
pub const FSTAT: usize = 189;
pub const LSTAT: usize = 190;
pub const PATHCONF: usize = 191;
pub const FPATHCONF: usize = 192;
pub const GETRLIMIT: usize = 194;
pub const SETRLIMIT: usize = 195;
pub const GETDIRENTRIES: usize = 196;
pub const FREEBSD6_MMAP: usize = 197;
pub const _SYSCALL: usize = 198;
pub const FREEBSD6_LSEEK: usize = 199;
pub const FREEBSD6_TRUNCATE: usize = 200;
pub const FREEBSD6_FTRUNCATE: usize = 201;
pub const SYSCTL: usize = 202;
pub const MLOCK: usize = 203;
pub const MUNLOCK: usize = 204;
pub const UNDELETE: usize = 205;
pub const FUTIMES: usize = 206;
pub const GETPGID: usize = 207;
pub const POLL: usize = 209;
pub const FREEBSD7___SEMCTL: usize = 220;
pub const SEMGET: usize = 221;
pub const SEMOP: usize = 222;
pub const FREEBSD7_MSGCTL: usize = 224;
pub const MSGGET: usize = 225;
pub const MSGSND: usize = 226;
pub const MSGRCV: usize = 227;
pub const SHMAT: usize = 228;
pub const FREEBSD7_SHMCTL: usize = 229;
pub const SHMDT: usize = 230;
pub const SHMGET: usize = 231;
pub const CLOCK_GETTIME: usize = 232;
pub const CLOCK_SETTIME: usize = 233;
pub const CLOCK_GETRES: usize = 234;
pub const KTIMER_CREATE: usize = 235;
pub const KTIMER_DELETE: usize = 236;
pub const KTIMER_SETTIME: usize = 237;
pub const KTIMER_GETTIME: usize = 238;
pub const KTIMER_GETOVERRUN: usize = 239;
pub const NANOSLEEP: usize = 240;
pub const FFCLOCK_GETCOUNTER: usize = 241;
pub const FFCLOCK_SETESTIMATE: usize = 242;
pub const FFCLOCK_GETESTIMATE: usize = 243;
pub const CLOCK_NANOSLEEP: usize = 244;
pub const CLOCK_GETCPUCLOCKID2: usize = 247;
pub const NTP_GETTIME: usize = 248;
pub const MINHERIT: usize = 250;
pub const RFORK: usize = 251;
pub const OPENBSD_POLL: usize = 252;
pub const ISSETUGID: usize = 253;
pub const LCHOWN: usize = 254;
pub const AIO_READ: usize = 255;
pub const AIO_WRITE: usize = 256;
pub const LIO_LISTIO: usize = 257;
pub const GETDENTS: usize = 272;
pub const LCHMOD: usize = 274;
pub const NETBSD_LCHOWN: usize = 275;
pub const LUTIMES: usize = 276;
pub const NETBSD_MSYNC: usize = 277;
pub const NSTAT: usize = 278;
pub const NFSTAT: usize = 279;
pub const NLSTAT: usize = 280;
pub const PREADV: usize = 289;
pub const PWRITEV: usize = 290;
pub const FREEBSD4_FHSTATFS: usize = 297;
pub const FHOPEN: usize = 298;
pub const FHSTAT: usize = 299;
pub const MODNEXT: usize = 300;
pub const MODSTAT: usize = 301;
pub const MODFNEXT: usize = 302;
pub const MODFIND: usize = 303;
pub const KLDLOAD: usize = 304;
pub const KLDUNLOAD: usize = 305;
pub const KLDFIND: usize = 306;
pub const KLDNEXT: usize = 307;
pub const KLDSTAT: usize = 308;
pub const KLDFIRSTMOD: usize = 309;
pub const GETSID: usize = 310;
pub const SETRESUID: usize = 311;
pub const SETRESGID: usize = 312;
pub const AIO_RETURN: usize = 314;
pub const AIO_SUSPEND: usize = 315;
pub const AIO_CANCEL: usize = 316;
pub const AIO_ERROR: usize = 317;
pub const OAIO_READ: usize = 318;
pub const OAIO_WRITE: usize = 319;
pub const OLIO_LISTIO: usize = 320;
pub const YIELD: usize = 321;
pub const MLOCKALL: usize = 324;
pub const MUNLOCKALL: usize = 325;
pub const GETCWD: usize = 326;
pub const SCHED_SETPARAM: usize = 327;
pub const SCHED_GETPARAM: usize = 328;
pub const SCHED_SETSCHEDULER: usize = 329;
pub const SCHED_GETSCHEDULER: usize = 330;
pub const SCHED_YIELD: usize = 331;
pub const SCHED_GET_PRIORITY_MAX: usize = 332;
pub const SCHED_GET_PRIORITY_MIN: usize = 333;
pub const SCHED_RR_GET_INTERVAL: usize = 334;
pub const UTRACE: usize = 335;
pub const FREEBSD4_SENDFILE: usize = 336;
pub const KLDSYM: usize = 337;
pub const JAIL: usize = 338;
pub const NNPFS_SYSCALL: usize = 339;
pub const SIGPROCMASK: usize = 340;
pub const SIGSUSPEND: usize = 341;
pub const FREEBSD4_SIGACTION: usize = 342;
pub const SIGPENDING: usize = 343;
pub const FREEBSD4_SIGRETURN: usize = 344;
pub const SIGTIMEDWAIT: usize = 345;
pub const SIGWAITINFO: usize = 346;
pub const ACL_GET_FILE: usize = 347;
pub const ACL_SET_FILE: usize = 348;
pub const ACL_GET_FD: usize = 349;
pub const ACL_SET_FD: usize = 350;
pub const ACL_DELETE_FILE: usize = 351;
pub const ACL_DELETE_FD: usize = 352;
pub const ACL_ACLCHECK_FILE: usize = 353;
pub const ACL_ACLCHECK_FD: usize = 354;
pub const EXTATTRCTL: usize = 355;
pub const EXTATTR_SET_FILE: usize = 356;
pub const EXTATTR_GET_FILE: usize = 357;
pub const EXTATTR_DELETE_FILE: usize = 358;
pub const AIO_WAITCOMPLETE: usize = 359;
pub const GETRESUID: usize = 360;
pub const GETRESGID: usize = 361;
pub const KQUEUE: usize = 362;
pub const KEVENT: usize = 363;
pub const EXTATTR_SET_FD: usize = 371;
pub const EXTATTR_GET_FD: usize = 372;
pub const EXTATTR_DELETE_FD: usize = 373;
pub const SETUGID: usize = 374;
pub const EACCESS: usize = 376;
pub const AFS3_SYSCALL: usize = 377;
pub const NMOUNT: usize = 378;
pub const MAC_GET_PROC: usize = 384;
pub const MAC_SET_PROC: usize = 385;
pub const MAC_GET_FD: usize = 386;
pub const MAC_GET_FILE: usize = 387;
pub const MAC_SET_FD: usize = 388;
pub const MAC_SET_FILE: usize = 389;
pub const KENV: usize = 390;
pub const LCHFLAGS: usize = 391;
pub const UUIDGEN: usize = 392;
pub const SENDFILE: usize = 393;
pub const MAC_SYSCALL: usize = 394;
pub const GETFSSTAT: usize = 395;
pub const STATFS: usize = 396;
pub const FSTATFS: usize = 397;
pub const FHSTATFS: usize = 398;
pub const KSEM_CLOSE: usize = 400;
pub const KSEM_POST: usize = 401;
pub const KSEM_WAIT: usize = 402;
pub const KSEM_TRYWAIT: usize = 403;
pub const KSEM_INIT: usize = 404;
pub const KSEM_OPEN: usize = 405;
pub const KSEM_UNLINK: usize = 406;
pub const KSEM_GETVALUE: usize = 407;
pub const KSEM_DESTROY: usize = 408;
pub const MAC_GET_PID: usize = 409;
pub const MAC_GET_LINK: usize = 410;
pub const MAC_SET_LINK: usize = 411;
pub const EXTATTR_SET_LINK: usize = 412;
pub const EXTATTR_GET_LINK: usize = 413;
pub const EXTATTR_DELETE_LINK: usize = 414;
pub const MAC_EXECVE: usize = 415;
pub const SIGACTION: usize = 416;
pub const SIGRETURN: usize = 417;
pub const GETCONTEXT: usize = 421;
pub const SETCONTEXT: usize = 422;
pub const SWAPCONTEXT: usize = 423;
pub const SWAPOFF: usize = 424;
pub const ACL_GET_LINK: usize = 425;
pub const ACL_SET_LINK: usize = 426;
pub const ACL_DELETE_LINK: usize = 427;
pub const ACL_ACLCHECK_LINK: usize = 428;
pub const SIGWAIT: usize = 429;
pub const THR_CREATE: usize = 430;
pub const THR_EXIT: usize = 431;
pub const THR_SELF: usize = 432;
pub const THR_KILL: usize = 433;
pub const _UMTX_LOCK: usize = 434;
pub const _UMTX_UNLOCK: usize = 435;
pub const JAIL_ATTACH: usize = 436;
pub const EXTATTR_LIST_FD: usize = 437;
pub const EXTATTR_LIST_FILE: usize = 438;
pub const EXTATTR_LIST_LINK: usize = 439;
pub const KSEM_TIMEDWAIT: usize = 441;
pub const THR_SUSPEND: usize = 442;
pub const THR_WAKE: usize = 443;
pub const KLDUNLOADF: usize = 444;
pub const AUDIT: usize = 445;
pub const AUDITON: usize = 446;
pub const GETAUID: usize = 447;
pub const SETAUID: usize = 448;
pub const GETAUDIT: usize = 449;
pub const SETAUDIT: usize = 450;
pub const GETAUDIT_ADDR: usize = 451;
pub const SETAUDIT_ADDR: usize = 452;
pub const AUDITCTL: usize = 453;
pub const _UMTX_OP: usize = 454;
pub const THR_NEW: usize = 455;
pub const SIGQUEUE: usize = 456;
pub const KMQ_OPEN: usize = 457;
pub const KMQ_SETATTR: usize = 458;
pub const KMQ_TIMEDRECEIVE: usize = 459;
pub const KMQ_TIMEDSEND: usize = 460;
pub const KMQ_NOTIFY: usize = 461;
pub const KMQ_UNLINK: usize = 462;
pub const ABORT2: usize = 463;
pub const THR_SET_NAME: usize = 464;
pub const AIO_FSYNC: usize = 465;
pub const RTPRIO_THREAD: usize = 466;
pub const SCTP_PEELOFF: usize = 471;
pub const SCTP_GENERIC_SENDMSG: usize = 472;
pub const SCTP_GENERIC_SENDMSG_IOV: usize = 473;
pub const SCTP_GENERIC_RECVMSG: usize = 474;
pub const PREAD: usize = 475;
pub const PWRITE: usize = 476;
pub const MMAP: usize = 477;
pub const LSEEK: usize = 478;
pub const TRUNCATE: usize = 479;
pub const FTRUNCATE: usize = 480;
pub const THR_KILL2: usize = 481;
pub const SHM_OPEN: usize = 482;
pub const SHM_UNLINK: usize = 483;
pub const CPUSET: usize = 484;
pub const CPUSET_SETID: usize = 485;
pub const CPUSET_GETID: usize = 486;
pub const CPUSET_GETAFFINITY: usize = 487;
pub const CPUSET_SETAFFINITY: usize = 488;
pub const FACCESSAT: usize = 489;
pub const FCHMODAT: usize = 490;
pub const FCHOWNAT: usize = 491;
pub const FEXECVE: usize = 492;
pub const FSTATAT: usize = 493;
pub const FUTIMESAT: usize = 494;
pub const LINKAT: usize = 495;
pub const MKDIRAT: usize = 496;
pub const MKFIFOAT: usize = 497;
pub const MKNODAT: usize = 498;
pub const OPENAT: usize = 499;
pub const READLINKAT: usize = 500;
pub const RENAMEAT: usize = 501;
pub const SYMLINKAT: usize = 502;
pub const UNLINKAT: usize = 503;
pub const POSIX_OPENPT: usize = 504;
pub const GSSD_SYSCALL: usize = 505;
pub const JAIL_GET: usize = 506;
pub const JAIL_SET: usize = 507;
pub const JAIL_REMOVE: usize = 508;
pub const CLOSEFROM: usize = 509;
pub const SEMCTL: usize = 510;
pub const MSGCTL: usize = 511;
pub const SHMCTL: usize = 512;
pub const LPATHCONF: usize = 513;
pub const CAP_RIGHTS_GET: usize = 515;
pub const CAP_ENTER: usize = 516;
pub const CAP_GETMODE: usize = 517;
pub const PDFORK: usize = 518;
pub const PDKILL: usize = 519;
pub const PDGETPID: usize = 520;
pub const PSELECT: usize = 522;
pub const GETLOGINCLASS: usize = 523;
pub const SETLOGINCLASS: usize = 524;
pub const RCTL_GET_RACCT: usize = 525;
pub const RCTL_GET_RULES: usize = 526;
pub const RCTL_GET_LIMITS: usize = 527;
pub const RCTL_ADD_RULE: usize = 528;
pub const RCTL_REMOVE_RULE: usize = 529;
pub const POSIX_FALLOCATE: usize = 530;
pub const POSIX_FADVISE: usize = 531;
pub const WAIT6: usize = 532;
pub const CAP_RIGHTS_LIMIT: usize = 533;
pub const CAP_IOCTLS_LIMIT: usize = 534;
pub const CAP_IOCTLS_GET: usize = 535;
pub const CAP_FCNTLS_LIMIT: usize = 536;
pub const CAP_FCNTLS_GET: usize = 537;
pub const BINDAT: usize = 538;
pub const CONNECTAT: usize = 539;
pub const CHFLAGSAT: usize = 540;
pub const ACCEPT4: usize = 541;
pub const PIPE2: usize = 542;
pub const AIO_MLOCK: usize = 543;
pub const PROCCTL: usize = 544;
pub const PPOLL: usize = 545;
pub const FUTIMENS: usize = 546;
pub const UTIMENSAT: usize = 547;
pub const FDATASYNC: usize = 550;
pub const CPUSET_GETDOMAIN: usize = 561;
pub const CPUSET_SETDOMAIN: usize = 562;
pub const GETRANDOM: usize = 563;
