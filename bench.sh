#!/usr/bin/env bash

# Benchmark 1: ators -i -o dwarf -l 0x0100360000 -f ./fixtures/many_addrs.txt
#   Time (mean ± σ):      63.8 ms ±   1.1 ms    [User: 59.1 ms, System: 3.3 ms]
#   Range (min … max):    61.6 ms …  65.9 ms    41 runs
#
# Benchmark 2: atos -i -o dwarf -l 0x0100360000 -f ./fixtures/many_addrs.txt
#   Time (mean ± σ):     227.0 ms ±   0.9 ms    [User: 211.7 ms, System: 11.3 ms]
#   Range (min … max):   225.7 ms … 228.5 ms    12 runs
#
# Benchmark 3: atosl -o dwarf -l 0 ...
#   Time (mean ± σ):      1.249 s ±  0.006 s    [User: 1.230 s, System: 0.016 s]
#   Range (min … max):    1.243 s …  1.263 s    10 runs
#
# Summary
#   'ators -i -o dwarf -l 0x0100360000 -f ./fixtures/many_addrs.txt' ran
#     3.56 ± 0.06 times faster than 'atos -i -o dwarf -l 0x0100360000 -f ./fixtures/many_addrs.txt'
#    19.58 ± 0.34 times faster than 'atosl -o dwarf -l 0 ...

hyperfine --warmup 3 \
'./target/release/ators -i -o ./fixtures/iosAppSwift.app.dSYM/Contents/Resources/DWARF/iosAppSwift -l 0x0100360000 -f ./fixtures/many_addrs.txt' \
'atos -i -o ./fixtures/iosAppSwift.app.dSYM/Contents/Resources/DWARF/iosAppSwift -l 0x0100360000 -f ./fixtures/many_addrs.txt' \
'../atosl-rs/target/release/atosl -o ./fixtures/iosAppSwift.app.dSYM/Contents/Resources/DWARF/iosAppSwift -l 0 0x6da8 0x6dac 0x6db0 0x6db4 0x6db8 0x6dbc 0x6dc0 0x6dc4 0x6dc8 0x6dcc 0x6dd0 0x6dd4 0x6dd8 0x6ddc 0x6de0 0x6de4 0x6de8 0x6dec 0x6df0 0x6df4 0x6df8 0x6dfc 0x6e00 0x6e04 0x6e08 0x6e0c 0x6e10 0x6e14 0x6e18 0x6e1c 0x6e20 0x6e24 0x6e28 0x6e2c 0x6e30 0x6e34 0x6e38 0x6e3c 0x6e40 0x6e44 0x6e48 0x6e4c 0x6e50 0x6e54 0x6e58 0x6e5c 0x6e60 0x6e64 0x6e68 0x6e6c 0x6e70 0x6e74 0x6e78 0x6e7c 0x6e80 0x6e84 0x6e88 0x6e8c 0x6e90 0x6e94 0x6e98 0x6e9c 0x6ea0 0x6ea4 0x6ea8 0x6eac 0x6eb0 0x6eb4 0x6eb8 0x6ebc 0x6ec0 0x6ec4 0x6ec8 0x6ecc 0x6ed0 0x6ed4 0x6ed8 0x6edc 0x6ee0 0x6ee4 0x6ee8 0x6eec 0x6ef0 0x6ef4 0x6ef8 0x6efc 0x6f00 0x6f04 0x6f08 0x6f0c 0x6f10 0x6f14 0x6f18 0x6f1c 0x6f20 0x6f24 0x6f28 0x6f2c 0x6f30 0x6f34 0x6f38 0x6f3c 0x6f40 0x6f44 0x6f48 0x6f4c 0x6f50 0x6f54 0x6f58 0x6f5c 0x6f60 0x6f64 0x6f68 0x6f6c 0x6f70 0x6f74 0x6f78 0x6f7c 0x6f80 0x6f84 0x6f88 0x6f8c 0x6f90 0x6f94 0x6f98 0x6f9c 0x6fa0 0x6fa4 0x6fa8 0x6fac 0x6fb0 0x6fb4 0x6fb8 0x6fbc 0x6fc0 0x6fc4 0x6fc8 0x6fcc 0x6fd0 0x6fd4 0x6fd8 0x6fdc 0x6fe0 0x6fe4 0x6fe8 0x6fec 0x6ff0 0x6ff4 0x6ff8 0x6ffc 0x7000 0x7004 0x7008 0x700c 0x7010 0x7014 0x7018 0x701c 0x7020 0x7024 0x7028 0x702c 0x7030 0x7034 0x7038 0x703c 0x7040 0x7044 0x7048 0x704c 0x7050 0x7054 0x7058 0x705c 0x7060 0x7064 0x7068 0x706c 0x7070 0x7074 0x7078 0x707c 0x7080 0x7084 0x7088 0x708c 0x7090 0x7094 0x7098 0x709c 0x70a0 0x70a4 0x70a8 0x70ac 0x70b0 0x70b4 0x70b8 0x70bc 0x70c0 0x70c4 0x70c8 0x70cc 0x70d0 0x70d4 0x70d8 0x70dc 0x70e0 0x70e4 0x70e8 0x70ec 0x70f0 0x70f4 0x70f8 0x70fc 0x7100 0x7104 0x7108 0x710c 0x7110 0x7114 0x7118 0x711c 0x7120 0x7124 0x7128 0x712c 0x7130 0x7134 0x7138 0x713c 0x7140 0x7144 0x7148 0x714c 0x7150 0x7154 0x7158 0x715c 0x7160 0x7164 0x7168 0x716c 0x7170 0x7174 0x7178 0x717c 0x7180 0x7184 0x7188 0x718c 0x7190 0x7194 0x7198 0x719c 0x71a0 0x71a4 0x71a8 0x71ac 0x71b0 0x71b4 0x71b8 0x71bc 0x71c0 0x71c4 0x71c8 0x71cc 0x71d0 0x71d4 0x71d8 0x71dc 0x71e0 0x71e4 0x71e8 0x71ec 0x71f0 0x71f4 0x71f8 0x71fc 0x7200 0x7204 0x7208 0x720c 0x7210 0x7214 0x7218 0x721c 0x7220 0x7224 0x7228 0x722c 0x7230 0x7234 0x7238 0x723c 0x7240 0x7244 0x7248 0x724c 0x7250 0x7254 0x7258 0x725c 0x7260 0x7264 0x7268 0x726c 0x7270 0x7274 0x7278 0x727c 0x7280 0x7284 0x7288 0x728c 0x7290 0x7294 0x7298 0x729c 0x72a0 0x72a4 0x72a8 0x72ac 0x72b0 0x72b4 0x72b8 0x72bc 0x72c0 0x72c4 0x72c8 0x72cc 0x72d0 0x72d4 0x72d8 0x72dc 0x72e0 0x72e4 0x72e8 0x72ec 0x72f0 0x72f4 0x72f8 0x72fc 0x7300 0x7304 0x7308 0x730c 0x7310 0x7314 0x7318 0x731c 0x7320 0x7324 0x7328 0x732c 0x7330 0x7334 0x7338 0x733c 0x7340 0x7344 0x7348 0x734c 0x7350 0x7354 0x7358 0x735c 0x7360 0x7364 0x7368 0x736c 0x7370 0x7374 0x7378 0x737c 0x7380 0x7384 0x7388 0x738c 0x7390 0x7394 0x7398 0x739c 0x73a0 0x73a4 0x73a8 0x73ac 0x73b0 0x73b4 0x73b8 0x73bc 0x73c0 0x73c4 0x73c8 0x73cc 0x73d0 0x73d4 0x73d8 0x73dc 0x73e0 0x73e4 0x73e8 0x73ec 0x73f0 0x73f4 0x73f8 0x73fc 0x7400 0x7404 0x7408 0x740c 0x7410 0x7414 0x7418 0x741c 0x7420 0x7424 0x7428 0x742c 0x7430 0x7434 0x7438 0x743c 0x7440 0x7444 0x7448 0x744c 0x7450 0x7454 0x7458 0x745c 0x7460 0x7464 0x7468 0x746c 0x7470 0x7474 0x7478 0x747c 0x7480 0x7484 0x7488 0x748c 0x7490 0x7494 0x7498 0x749c 0x74a0 0x74a4 0x74a8 0x74ac 0x74b0 0x74b4 0x74b8 0x74bc 0x74c0 0x74c4 0x74c8 0x74cc 0x74d0 0x74d4 0x74d8 0x74dc 0x74e0 0x74e4 0x74e8 0x74ec 0x74f0 0x74f4 0x74f8 0x74fc 0x7500 0x7504 0x7508 0x750c 0x7510 0x7514 0x7518 0x751c 0x7520 0x7524 0x7528 0x752c 0x7530 0x7534 0x7538 0x753c 0x7540 0x7544 0x7548 0x754c 0x7550 0x7554 0x7558 0x755c 0x7560 0x7564 0x7568 0x756c 0x7570 0x7574 0x7578 0x757c 0x7580 0x7584 0x7588 0x758c 0x7590 0x7594 0x7598 0x759c 0x75a0 0x75a4 0x75a8 0x75ac 0x75b0 0x75b4 0x75b8 0x75bc 0x75c0 0x75c4 0x75c8 0x75cc 0x75d0 0x75d4 0x75d8 0x75dc 0x75e0 0x75e4 0x75e8 0x75ec 0x75f0 0x75f4 0x75f8 0x75fc 0x7600 0x7604 0x7608 0x760c 0x7610 0x7614 0x7618 0x761c 0x7620 0x7624 0x7628 0x762c 0x7630 0x7634 0x7638 0x763c 0x7640 0x7644 0x7648 0x764c 0x7650 0x7654 0x7658 0x765c 0x7660 0x7664 0x7668 0x766c 0x7670 0x7674 0x7678 0x767c 0x7680 0x7684 0x7688 0x768c 0x7690 0x7694 0x7698 0x769c 0x76a0 0x76a4 0x76a8 0x76ac 0x76b0 0x76b4 0x76b8 0x76bc 0x76c0 0x76c4 0x76c8 0x76cc 0x76d0 0x76d4 0x76d8 0x76dc 0x76e0 0x76e4 0x76e8 0x76ec 0x76f0 0x76f4 0x76f8 0x76fc 0x7700 0x7704 0x7708 0x770c 0x7710 0x7714 0x7718 0x771c 0x7720 0x7724 0x7728 0x772c 0x7730 0x7734 0x7738 0x773c 0x7740 0x7744 0x7748 0x774c 0x7750 0x7754 0x7758 0x775c 0x7760 0x7764 0x7768 0x776c 0x7770 0x7774 0x7778 0x777c 0x7780 0x7784 0x7788 0x778c 0x7790 0x7794 0x7798 0x779c 0x77a0 0x77a4 0x77a8 0x77ac 0x77b0 0x77b4 0x77b8 0x77bc 0x77c0 0x77c4 0x77c8 0x77cc 0x77d0 0x77d4 0x77d8 0x77dc 0x77e0 0x77e4 0x77e8 0x77ec 0x77f0 0x77f4 0x77f8 0x77fc 0x7800 0x7804 0x7808 0x780c 0x7810 0x7814 0x7818 0x781c 0x7820 0x7824 0x7828 0x782c 0x7830 0x7834 0x7838 0x783c 0x7840 0x7844 0x7848 0x784c 0x7850 0x7854 0x7858 0x785c 0x7860 0x7864 0x7868 0x786c 0x7870 0x7874 0x7878 0x787c 0x7880 0x7884 0x7888 0x788c 0x7890 0x7894 0x7898 0x789c 0x78a0 0x78a4 0x78a8 0x78ac 0x78b0 0x78b4 0x78b8 0x78bc 0x78c0 0x78c4 0x78c8 0x78cc 0x78d0 0x78d4 0x78d8 0x78dc 0x78e0 0x78e4 0x78e8 0x78ec 0x78f0 0x78f4 0x78f8 0x78fc 0x7900 0x7904 0x7908 0x790c 0x7910 0x7914 0x7918 0x791c 0x7920 0x7924 0x7928 0x792c 0x7930 0x7934 0x7938 0x793c 0x7940 0x7944 0x7948 0x794c 0x7950 0x7954 0x7958 0x795c 0x7960 0x7964 0x7968 0x796c 0x7970 0x7974 0x7978 0x797c 0x7980 0x7984 0x7988 0x798c 0x7990 0x7994 0x7998 0x799c 0x79a0 0x79a4 0x79a8 0x79ac 0x79b0 0x79b4 0x79b8 0x79bc 0x79c0 0x79c4 0x79c8 0x79cc 0x79d0 0x79d4 0x79d8 0x79dc 0x79e0 0x79e4 0x79e8 0x79ec 0x79f0 0x79f4 0x79f8 0x79fc 0x7a00 0x7a04 0x7a08 0x7a0c 0x7a10 0x7a14 0x7a18 0x7a1c 0x7a20 0x7a24 0x7a28 0x7a2c 0x7a30 0x7a34 0x7a38 0x7a3c 0x7a40 0x7a44 0x7a48 0x7a4c 0x7a50 0x7a54 0x7a58 0x7a5c 0x7a60 0x7a64 0x7a68 0x7a6c 0x7a70 0x7a74 0x7a78 0x7a7c 0x7a80 0x7a84 0x7a88 0x7a8c 0x7a90 0x7a94 0x7a98 0x7a9c 0x7aa0 0x7aa4 0x7aa8 0x7aac 0x7ab0 0x7ab4 0x7ab8 0x7abc 0x7ac0 0x7ac4 0x7ac8 0x7acc 0x7ad0 0x7ad4 0x7ad8 0x7adc 0x7ae0 0x7ae4 0x7ae8 0x7aec 0x7af0 0x7af4 0x7af8 0x7afc 0x7b00 0x7b04 0x7b08 0x7b0c 0x7b10 0x7b14 0x7b18 0x7b1c 0x7b20 0x7b24 0x7b28 0x7b2c 0x7b30 0x7b34 0x7b38 0x7b3c 0x7b40 0x7b44 0x7b48 0x7b4c 0x7b50 0x7b54 0x7b58 0x7b5c 0x7b60 0x7b64 0x7b68 0x7b6c 0x7b70 0x7b74 0x7b78 0x7b7c 0x7b80 0x7b84 0x7b88 0x7b8c 0x7b90 0x7b94 0x7b98 0x7b9c 0x7ba0 0x7ba4 0x7ba8 0x7bac 0x7bb0 0x7bb4 0x7bb8 0x7bbc 0x7bc0 0x7bc4 0x7bc8 0x7bcc 0x7bd0 0x7bd4 0x7bd8 0x7bdc 0x7be0 0x7be4 0x7be8 0x7bec 0x7bf0 0x7bf4 0x7bf8 0x7bfc 0x7c00 0x7c04 0x7c08 0x7c0c 0x7c10 0x7c14 0x7c18 0x7c1c 0x7c20 0x7c24 0x7c28 0x7c2c 0x7c30 0x7c34 0x7c38 0x7c3c 0x7c40 0x7c44 0x7c48 0x7c4c 0x7c50 0x7c54 0x7c58 0x7c5c 0x7c60 0x7c64 0x7c68 0x7c6c 0x7c70 0x7c74 0x7c78 0x7c7c 0x7c80 0x7c84 0x7c88 0x7c8c 0x7c90 0x7c94 0x7c98 0x7c9c 0x7ca0 0x7ca4 0x7ca8 0x7cac 0x7cb0 0x7cb4 0x7cb8 0x7cbc 0x7cc0 0x7cc4 0x7cc8 0x7ccc 0x7cd0 0x7cd4 0x7cd8 0x7cdc 0x7ce0 0x7ce4 0x7ce8 0x7cec 0x7cf0 0x7cf4 0x7cf8 0x7cfc 0x7d00 0x7d04 0x7d08 0x7d0c 0x7d10 0x7d14 0x7d18 0x7d1c 0x7d20 0x7d24 0x7d28 0x7d2c 0x7d30 0x7d34 0x7d38 0x7d3c 0x7d40 0x7d44 0x7d48 0x7d4c 0x7d50 0x7d54 0x7d58 0x7d5c 0x7d60 0x7d64 0x7d68 0x7d6c 0x7d70 0x7d74 0x7d78 0x7d7c 0x7d80 0x7d84 0x7d88 0x7d8c 0x7d90 0x7d94 0x7d98 0x7d9c 0x7da0 0x7da4 0x7da8 0x7dac 0x7db0 0x7db4 0x7db8 0x7dbc 0x7dc0 0x7dc4 0x7dc8 0x7dcc 0x7dd0 0x7dd4 0x7dd8 0x7ddc 0x7de0 0x7de4 0x7de8 0x7dec 0x7df0 0x7df4 0x7df8 0x7dfc 0x7e00 0x7e04 0x7e08 0x7e0c 0x7e10 0x7e14 0x7e18 0x7e1c 0x7e20 0x7e24 0x7e28 0x7e2c 0x7e30 0x7e34 0x7e38 0x7e3c 0x7e40 0x7e44 0x7e48 0x7e4c 0x7e50 0x7e54 0x7e58 0x7e5c 0x7e60 0x7e64 0x7e68 0x7e6c 0x7e70 0x7e74 0x7e78 0x7e7c 0x7e80 0x7e84 0x7e88 0x7e8c 0x7e90 0x7e94 0x7e98 0x7e9c 0x7ea0 0x7ea4 0x7ea8 0x7eac 0x7eb0 0x7eb4 0x7eb8 0x7ebc 0x7ec0 0x7ec4 0x7ec8 0x7ecc 0x7ed0 0x7ed4 0x7ed8 0x7edc 0x7ee0 0x7ee4 0x7ee8 0x7eec 0x7ef0 0x7ef4 0x7ef8 0x7efc 0x7f00 0x7f04 0x7f08 0x7f0c 0x7f10 0x7f14 0x7f18 0x7f1c 0x7f20 0x7f24 0x7f28 0x7f2c 0x7f30 0x7f34 0x7f38 0x7f3c 0x7f40 0x7f44 0x7f48 0x7f4c 0x7f50 0x7f54 0x7f58 0x7f5c 0x7f60 0x7f64 0x7f68 0x7f6c 0x7f70 0x7f74 0x7f78 0x7f7c 0x7f80 0x7f84 0x7f88 0x7f8c 0x7f90 0x7f94 0x7f98 0x7f9c 0x7fa0 0x7fa4 0x7fa8 0x7fac 0x7fb0 0x7fb4 0x7fb8 0x7fbc 0x7fc0 0x7fc4 0x7fc8 0x7fcc 0x7fd0 0x7fd4 0x7fd8 0x7fdc 0x7fe0 0x7fe4 0x7fe8 0x7fec 0x7ff0 0x7ff4 0x7ff8 0x7ffc 0x8000 0x8004 0x8008 0x800c 0x8010 0x8014 0x8018 0x801c 0x8020 0x8024 0x8028 0x802c 0x8030 0x8034 0x8038 0x803c 0x8040 0x8044 0x8048 0x804c 0x8050 0x8054 0x8058 0x805c 0x8060 0x8064 0x8068 0x806c 0x8070 0x8074 0x8078 0x807c 0x8080 0x8084 0x8088 0x808c 0x8090 0x8094 0x8098 0x809c 0x80a0 0x80a4 0x80a8 0x80ac 0x80b0 0x80b4 0x80b8 0x80bc 0x80c0 0x80c4 0x80c8 0x80cc 0x80d0 0x80d4 0x80d8 0x80dc 0x80e0 0x80e4 0x80e8 0x80ec 0x80f0 0x80f4 0x80f8 0x80fc 0x8100 0x8104 0x8108 0x810c 0x8110 0x8114 0x8118 0x811c 0x8120 0x8124 0x8128 0x812c 0x8130 0x8134 0x8138 0x813c 0x8140 0x8144 0x8148 0x814c 0x8150 0x8154 0x8158 0x815c 0x8160 0x8164 0x8168 0x816c 0x8170 0x8174 0x8178 0x817c 0x8180 0x8184 0x8188 0x818c 0x8190 0x8194 0x8198 0x819c 0x81a0 0x81a4 0x81a8 0x81ac 0x81b0 0x81b4 0x81b8 0x81bc 0x81c0 0x81c4 0x81c8 0x81cc 0x81d0 0x81d4 0x81d8 0x81dc 0x81e0 0x81e4 0x81e8 0x81ec 0x81f0 0x81f4 0x81f8 0x81fc 0x8200 0x8204 0x8208 0x820c 0x8210 0x8214 0x8218 0x821c 0x8220 0x8224 0x8228 0x822c 0x8230 0x8234 0x8238 0x823c 0x8240 0x8244 0x8248 0x824c 0x8250 0x8254 0x8258 0x825c 0x8260 0x8264 0x8268 0x826c 0x8270 0x8274 0x8278 0x827c 0x8280 0x8284 0x8288 0x828c 0x8290 0x8294 0x8298 0x829c 0x82a0 0x82a4 0x82a8 0x82ac 0x82b0 0x82b4 0x82b8 0x82bc 0x82c0 0x82c4 0x82c8 0x82cc 0x82d0 0x82d4 0x82d8 0x82dc 0x82e0 0x82e4 0x82e8 0x82ec 0x82f0 0x82f4 0x82f8 0x82fc 0x8300 0x8304 0x8308 0x830c 0x8310 0x8314 0x8318 0x831c 0x8320 0x8324 0x8328 0x832c 0x8330 0x8334 0x8338 0x833c 0x8340 0x8344 0x8348 0x834c 0x8350 0x8354 0x8358 0x835c 0x8360 0x8364 0x8368 0x836c 0x8370 0x8374 0x8378 0x837c 0x8380 0x8384 0x8388 0x838c 0x8390 0x8394 0x8398 0x839c 0x83a0 0x83a4 0x83a8 0x83ac 0x83b0 0x83b4 0x83b8 0x83bc 0x83c0 0x83c4 0x83c8 0x83cc 0x83d0 0x83d4 0x83d8 0x83dc 0x83e0 0x83e4 0x83e8 0x83ec 0x83f0 0x83f4 0x83f8 0x83fc 0x8400 0x8404 0x8408 0x840c 0x8410 0x8414 0x8418 0x841c 0x8420 0x8424 0x8428 0x842c 0x8430 0x8434 0x8438 0x843c 0x8440 0x8444 0x8448 0x844c 0x8450 0x8454 0x8458 0x845c 0x8460 0x8464 0x8468 0x846c 0x8470 0x8474 0x8478 0x847c 0x8480 0x8484 0x8488 0x848c 0x8490 0x8494 0x8498 0x849c 0x84a0 0x84a4 0x84a8 0x84ac 0x84b0 0x84b4 0x84b8 0x84bc 0x84c0 0x84c4 0x84c8 0x84cc 0x84d0 0x84d4 0x84d8 0x84dc 0x84e0 0x84e4 0x84e8 0x84ec 0x84f0 0x84f4 0x84f8 0x84fc 0x8500 0x8504 0x8508 0x850c 0x8510 0x8514 0x8518 0x851c 0x8520 0x8524 0x8528 0x852c 0x8530 0x8534 0x8538 0x853c 0x8540 0x8544 0x8548 0x854c 0x8550 0x8554 0x8558 0x855c 0x8560 0x8564 0x8568 0x856c 0x8570 0x8574 0x8578 0x857c 0x8580 0x8584 0x8588 0x858c 0x8590 0x8594 0x8598 0x859c 0x85a0 0x85a4 0x85a8 0x85ac 0x85b0 0x85b4 0x85b8 0x85bc 0x85c0 0x85c4 0x85c8 0x85cc 0x85d0 0x85d4 0x85d8 0x85dc 0x85e0 0x85e4 0x85e8 0x85ec 0x85f0 0x85f4 0x85f8 0x85fc 0x8600 0x8604 0x8608 0x860c 0x8610 0x8614 0x8618 0x861c 0x8620 0x8624 0x8628 0x862c 0x8630 0x8634 0x8638 0x863c 0x8640 0x8644 0x8648 0x864c 0x8650 0x8654 0x8658 0x865c 0x8660 0x8664 0x8668 0x866c 0x8670 0x8674 0x8678 0x867c 0x8680 0x8684 0x8688 0x868c 0x8690 0x8694 0x8698 0x869c 0x86a0 0x86a4 0x86a8 0x86ac 0x86b0 0x86b4 0x86b8 0x86bc 0x86c0 0x86c4 0x86c8 0x86cc 0x86d0 0x86d4 0x86d8 0x86dc 0x86e0 0x86e4 0x86e8 0x86ec 0x86f0 0x86f4 0x86f8 0x86fc 0x8700 0x8704 0x8708 0x870c 0x8710 0x8714 0x8718 0x871c 0x8720 0x8724 0x8728 0x872c 0x8730 0x8734 0x8738 0x873c 0x8740 0x8744 0x8748 0x874c 0x8750 0x8754 0x8758 0x875c 0x8760 0x8764 0x8768 0x876c 0x8770 0x8774 0x8778 0x877c 0x8780 0x8784 0x8788 0x878c 0x8790 0x8794 0x8798 0x879c 0x87a0 0x87a4 0x87a8 0x87ac 0x87b0 0x87b4 0x87b8 0x87bc 0x87c0 0x87c4 0x87c8 0x87cc 0x87d0 0x87d4 0x87d8 0x87dc 0x87e0 0x87e4 0x87e8 0x87ec 0x87f0 0x87f4 0x87f8 0x87fc 0x8800 0x8804 0x8808 0x880c 0x8810 0x8814 0x8818 0x881c 0x8820 0x8824 0x8828 0x882c 0x8830 0x8834 0x8838 0x883c 0x8840 0x8844 0x8848 0x884c 0x8850 0x8854 0x8858 0x885c 0x8860 0x8864 0x8868 0x886c 0x8870 0x8874 0x8878 0x887c 0x8880 0x8884 0x8888 0x888c 0x8890 0x8894 0x8898 0x889c 0x88a0 0x88a4 0x88a8 0x88ac 0x88b0 0x88b4 0x88b8 0x88bc 0x88c0 0x88c4 0x88c8 0x88cc 0x88d0 0x88d4 0x88d8 0x88dc 0x88e0 0x88e4 0x88e8 0x88ec 0x88f0 0x88f4 0x88f8 0x88fc 0x8900 0x8904 0x8908 0x890c 0x8910 0x8914 0x8918 0x891c 0x8920 0x8924 0x8928 0x892c 0x8930 0x8934 0x8938 0x893c 0x8940 0x8944 0x8948 0x894c 0x8950 0x8954 0x8958 0x895c 0x8960 0x8964 0x8968 0x896c 0x8970 0x8974 0x8978 0x897c 0x8980 0x8984 0x8988 0x898c 0x8990 0x8994 0x8998 0x899c 0x89a0 0x89a4 0x89a8 0x89ac 0x89b0 0x89b4 0x89b8 0x89bc 0x89c0 0x89c4 0x89c8 0x89cc 0x89d0 0x89d4 0x89d8 0x89dc 0x89e0 0x89e4 0x89e8 0x89ec 0x89f0 0x89f4 0x89f8 0x89fc 0x8a00 0x8a04 0x8a08 0x8a0c 0x8a10 0x8a14 0x8a18 0x8a1c 0x8a20 0x8a24 0x8a28 0x8a2c 0x8a30 0x8a34 0x8a38 0x8a3c 0x8a40 0x8a44 0x8a48 0x8a4c 0x8a50 0x8a54 0x8a58 0x8a5c 0x8a60 0x8a64 0x8a68 0x8a6c 0x8a70 0x8a74 0x8a78 0x8a7c 0x8a80 0x8a84 0x8a88 0x8a8c 0x8a90 0x8a94 0x8a98 0x8a9c 0x8aa0 0x8aa4 0x8aa8 0x8aac 0x8ab0 0x8ab4 0x8ab8 0x8abc 0x8ac0 0x8ac4 0x8ac8 0x8acc 0x8ad0 0x8ad4 0x8ad8 0x8adc 0x8ae0 0x8ae4 0x8ae8 0x8aec 0x8af0 0x8af4 0x8af8 0x8afc 0x8b00 0x8b04 0x8b08 0x8b0c 0x8b10 0x8b14 0x8b18 0x8b1c 0x8b20 0x8b24 0x8b28 0x8b2c 0x8b30 0x8b34 0x8b38 0x8b3c 0x8b40 0x8b44 0x8b48 0x8b4c 0x8b50 0x8b54 0x8b58 0x8b5c 0x8b60 0x8b64 0x8b68 0x8b6c 0x8b70 0x8b74 0x8b78 0x8b7c 0x8b80 0x8b84 0x8b88 0x8b8c 0x8b90 0x8b94 0x8b98 0x8b9c 0x8ba0 0x8ba4 0x8ba8 0x8bac 0x8bb0 0x8bb4 0x8bb8 0x8bbc 0x8bc0 0x8bc4 0x8bc8 0x8bcc 0x8bd0 0x8bd4 0x8bd8 0x8bdc 0x8be0 0x8be4 0x8be8 0x8bec 0x8bf0 0x8bf4 0x8bf8 0x8bfc 0x8c00 0x8c04 0x8c08 0x8c0c 0x8c10 0x8c14 0x8c18 0x8c1c 0x8c20 0x8c24 0x8c28 0x8c2c 0x8c30 0x8c34 0x8c38 0x8c3c 0x8c40 0x8c44 0x8c48 0x8c4c 0x8c50 0x8c54 0x8c58 0x8c5c 0x8c60 0x8c64 0x8c68 0x8c6c 0x8c70 0x8c74 0x8c78 0x8c7c 0x8c80 0x8c84 0x8c88 0x8c8c 0x8c90 0x8c94 0x8c98 0x8c9c 0x8ca0 0x8ca4 0x8ca8 0x8cac 0x8cb0 0x8cb4 0x8cb8 0x8cbc 0x8cc0 0x8cc4 0x8cc8 0x8ccc 0x8cd0 0x8cd4 0x8cd8 0x8cdc 0x8ce0 0x8ce4 0x8ce8 0x8cec 0x8cf0 0x8cf4 0x8cf8 0x8cfc 0x8d00 0x8d04 0x8d08 0x8d0c 0x8d10 0x8d14 0x8d18 0x8d1c 0x8d20 0x8d24 0x8d28 0x8d2c 0x8d30 0x8d34 0x8d38 0x8d3c 0x8d40 0x8d44 0x8d48 0x8d4c 0x8d50 0x8d54 0x8d58 0x8d5c 0x8d60 0x8d64 0x8d68 0x8d6c 0x8d70 0x8d74 0x8d78 0x8d7c 0x8d80 0x8d84 0x8d88 0x8d8c 0x8d90 0x8d94 0x8d98 0x8d9c 0x8da0 0x8da4 0x8da8 0x8dac 0x8db0 0x8db4 0x8db8 0x8dbc 0x8dc0 0x8dc4 0x8dc8 0x8dcc 0x8dd0 0x8dd4 0x8dd8 0x8ddc 0x8de0 0x8de4 0x8de8 0x8dec 0x8df0 0x8df4 0x8df8 0x8dfc 0x8e00 0x8e04 0x8e08 0x8e0c 0x8e10 0x8e14 0x8e18 0x8e1c 0x8e20 0x8e24 0x8e28 0x8e2c 0x8e30 0x8e34 0x8e38 0x8e3c 0x8e40 0x8e44 0x8e48 0x8e4c 0x8e50 0x8e54 0x8e58 0x8e5c 0x8e60 0x8e64 0x8e68 0x8e6c 0x8e70 0x8e74 0x8e78 0x8e7c 0x8e80 0x8e84 0x8e88 0x8e8c 0x8e90 0x8e94 0x8e98 0x8e9c 0x8ea0 0x8ea4 0x8ea8 0x8eac 0x8eb0 0x8eb4 0x8eb8 0x8ebc 0x8ec0 0x8ec4 0x8ec8 0x8ecc 0x8ed0 0x8ed4 0x8ed8 0x8edc 0x8ee0 0x8ee4 0x8ee8 0x8eec 0x8ef0 0x8ef4 0x8ef8 0x8efc 0x8f00 0x8f04 0x8f08 0x8f0c 0x8f10 0x8f14 0x8f18 0x8f1c 0x8f20 0x8f24 0x8f28 0x8f2c 0x8f30 0x8f34 0x8f38 0x8f3c 0x8f40 0x8f44 0x8f48 0x8f4c 0x8f50 0x8f54 0x8f58 0x8f5c 0x8f60 0x8f64 0x8f68 0x8f6c 0x8f70 0x8f74 0x8f78 0x8f7c 0x8f80 0x8f84 0x8f88 0x8f8c 0x8f90 0x8f94 0x8f98 0x8f9c 0x8fa0 0x8fa4 0x8fa8 0x8fac 0x8fb0 0x8fb4 0x8fb8 0x8fbc 0x8fc0 0x8fc4 0x8fc8 0x8fcc 0x8fd0 0x8fd4 0x8fd8 0x8fdc 0x8fe0 0x8fe4 0x8fe8 0x8fec 0x8ff0 0x8ff4 0x8ff8 0x8ffc 0x9000 0x9004 0x9008 0x900c 0x9010 0x9014 0x9018 0x901c 0x9020 0x9024 0x9028 0x902c 0x9030 0x9034 0x9038 0x903c 0x9040 0x9044 0x9048 0x904c 0x9050 0x9054 0x9058 0x905c 0x9060 0x9064 0x9068 0x906c 0x9070 0x9074 0x9078 0x907c 0x9080 0x9084 0x9088 0x908c 0x9090 0x9094 0x9098 0x909c 0x90a0 0x90a4 0x90a8 0x90ac 0x90b0 0x90b4 0x90b8 0x90bc 0x90c0 0x90c4 0x90c8 0x90cc 0x90d0 0x90d4 0x90d8 0x90dc 0x90e0 0x90e4 0x90e8 0x90ec 0x90f0 0x90f4 0x90f8 0x90fc 0x9100 0x9104 0x9108 0x910c 0x9110 0x9114 0x9118 0x911c 0x9120 0x9124 0x9128 0x912c 0x9130 0x9134 0x9138 0x913c 0x9140 0x9144 0x9148 0x914c 0x9150 0x9154 0x9158 0x915c 0x9160 0x9164 0x9168 0x916c 0x9170 0x9174 0x9178 0x917c 0x9180 0x9184 0x9188 0x918c 0x9190 0x9194 0x9198 0x919c 0x91a0 0x91a4 0x91a8 0x91ac 0x91b0 0x91b4 0x91b8 0x91bc 0x91c0 0x91c4 0x91c8 0x91cc 0x91d0 0x91d4 0x91d8 0x91dc 0x91e0 0x91e4 0x91e8 0x91ec 0x91f0 0x91f4 0x91f8 0x91fc 0x9200 0x9204 0x9208 0x920c 0x9210 0x9214 0x9218 0x921c 0x9220 0x9224 0x9228 0x922c 0x9230 0x9234 0x9238 0x923c 0x9240 0x9244 0x9248 0x924c 0x9250 0x9254 0x9258 0x925c 0x9260 0x9264 0x9268 0x926c 0x9270 0x9274 0x9278 0x927c 0x9280 0x9284 0x9288 0x928c 0x9290 0x9294 0x9298 0x929c 0x92a0 0x92a4 0x92a8 0x92ac 0x92b0 0x92b4 0x92b8 0x92bc 0x92c0 0x92c4 0x92c8 0x92cc 0x92d0 0x92d4 0x92d8 0x92dc 0x92e0 0x92e4 0x92e8 0x92ec 0x92f0 0x92f4 0x92f8 0x92fc 0x9300 0x9304 0x9308 0x930c 0x9310 0x9314 0x9318 0x931c 0x9320 0x9324 0x9328 0x932c 0x9330 0x9334 0x9338 0x933c 0x9340 0x9344 0x9348 0x934c 0x9350 0x9354 0x9358 0x935c 0x9360 0x9364 0x9368 0x936c 0x9370 0x9374 0x9378 0x937c 0x9380 0x9384 0x9388 0x938c 0x9390 0x9394 0x9398 0x939c 0x93a0 0x93a4 0x93a8 0x93ac 0x93b0 0x93b4 0x93b8 0x93bc 0x93c0 0x93c4 0x93c8 0x93cc 0x93d0 0x93d4 0x93d8 0x93dc 0x93e0 0x93e4 0x93e8 0x93ec 0x93f0 0x93f4 0x93f8 0x93fc 0x9400 0x9404 0x9408 0x940c 0x9410 0x9414 0x9418 0x941c 0x9420 0x9424 0x9428 0x942c 0x9430 0x9434 0x9438 0x943c 0x9440 0x9444 0x9448 0x944c 0x9450 0x9454 0x9458 0x945c 0x9460 0x9464 0x9468 0x946c 0x9470 0x9474 0x9478 0x947c 0x9480 0x9484 0x9488 0x948c 0x9490 0x9494 0x9498 0x949c 0x94a0 0x94a4 0x94a8 0x94ac 0x94b0 0x94b4 0x94b8 0x94bc 0x94c0 0x94c4 0x94c8 0x94cc 0x94d0 0x94d4 0x94d8 0x94dc 0x94e0 0x94e4 0x94e8 0x94ec 0x94f0 0x94f4 0x94f8 0x94fc 0x9500 0x9504 0x9508 0x950c 0x9510 0x9514 0x9518 0x951c 0x9520 0x9524 0x9528 0x952c 0x9530 0x9534 0x9538 0x953c 0x9540 0x9544 0x9548 0x954c 0x9550 0x9554 0x9558 0x955c 0x9560 0x9564 0x9568 0x956c 0x9570 0x9574 0x9578 0x957c 0x9580 0x9584 0x9588 0x958c 0x9590 0x9594 0x9598 0x959c 0x95a0 0x95a4 0x95a8 0x95ac 0x95b0 0x95b4 0x95b8 0x95bc 0x95c0 0x95c4 0x95c8 0x95cc 0x95d0 0x95d4 0x95d8 0x95dc 0x95e0 0x95e4 0x95e8 0x95ec 0x95f0 0x95f4 0x95f8 0x95fc 0x9600 0x9604 0x9608 0x960c 0x9610 0x9614 0x9618 0x961c 0x9620 0x9624 0x9628 0x962c 0x9630 0x9634 0x9638 0x963c 0x9640 0x9644 0x9648 0x964c 0x9650 0x9654 0x9658 0x965c 0x9660 0x9664 0x9668 0x966c 0x9670 0x9674 0x9678 0x967c 0x9680 0x9684 0x9688 0x968c 0x9690 0x9694 0x9698 0x969c 0x96a0 0x96a4 0x96a8 0x96ac 0x96b0 0x96b4 0x96b8 0x96bc 0x96c0 0x96c4 0x96c8 0x96cc 0x96d0 0x96d4 0x96d8 0x96dc 0x96e0 0x96e4 0x96e8 0x96ec 0x96f0 0x96f4 0x96f8 0x96fc 0x9700 0x9704 0x9708 0x970c 0x9710 0x9714 0x9718 0x971c 0x9720 0x9724 0x9728 0x972c 0x9730 0x9734 0x9738 0x973c 0x9740 0x9744 0x9748 0x974c 0x9750 0x9754 0x9758 0x975c 0x9760 0x9764 0x9768 0x976c 0x9770 0x9774 0x9778 0x977c 0x9780 0x9784 0x9788 0x978c 0x9790 0x9794 0x9798 0x979c 0x97a0 0x97a4 0x97a8 0x97ac 0x97b0 0x97b4 0x97b8 0x97c8 0x97cc 0x97d0 0x97d4 0x97d8 0x97dc 0x97e0 0x97e4 0x97e8 0x97ec 0x97f0 0x97f4 0x97f8 0x97fc 0x9800 0x9804 0x9808 0x980c 0x9810 0x9814 0x9818 0x981c 0x9820 0x9824 0x9828 0x982c 0x9830 0x9834 0x9838 0x983c 0x9840 0x9844 0x9848 0x984c 0x9850 0x9854 0x9858 0x985c 0x9860 0x9864 0x9868 0x986c 0x9870 0x9874 0x98c0 0x98c4 0x98c8 0x98cc 0x98d0 0x98d4 0x98d8 0x98dc 0x98e0 0x98e4 0x98e8 0x98ec 0x98f0 0x98f4 0x98f8 0x98fc 0x9900 0x9904 0x9908 0x990c 0x9910 0x9914 0x9918 0x991c 0x9920 0x9924 0x9928 0x992c 0x9930 0x9934 0x9938 0x993c 0x9940 0x9944 0x9948 0x994c 0x9950 0x9954 0x9958 0x995c 0x9960 0x9964 0x9968 0x996c 0x9970 0x9974 0x9978 0x997c 0x9980 0x9984 0x9988 0x998c 0x9990 0x9994 0x9998 0x999c 0x99a0 0x99a4 0x99a8 0x99ac 0x99b0 0x99b4 0x99b8 0x99bc 0x99c0 0x99c4 0x99c8 0x99cc 0x99d0 0x99d4 0x99d8 0x99dc 0x99e0 0x99e4 0x99e8 0x99ec 0x99f0 0x99f4 0x99f8 0x99fc 0x9a00 0x9a04 0x9a08 0x9a0c 0x9a10 0x9a14 0x9a18 0x9a1c 0x9a20 0x9a24 0x9a28 0x9a2c 0x9a30 0x9a34 0x9a38 0x9a3c 0x9a40 0x9a44 0x9a48 0x9a4c 0x9a50 0x9a54 0x9a58 0x9a5c 0x9a60 0x9a64 0x9a68 0x9a6c 0x9a70 0x9a74 0x9a78 0x9a7c 0x9a80 0x9a84 0x9a88 0x9a8c 0x9a90 0x9a94 0x9a98 0x9a9c 0x9aa0 0x9aa4 0x9aa8 0x9aac 0x9ab0 0x9ab4 0x9ab8 0x9abc 0x9ac0 0x9ac4 0x9ac8 0x9acc 0x9ad0 0x9ad4 0x9ad8 0x9adc 0x9ae0 0x9ae4 0x9ae8 0x9aec 0x9af0 0x9af4 0x9af8 0x9afc 0x9b00 0x9b04 0x9b08 0x9b0c 0x9b10 0x9b14 0x9b18 0x9b1c 0x9b20 0x9b24 0x9b28 0x9b2c 0x9b30 0x9b34 0x9b38 0x9b3c 0x9b40 0x9b44 0x9b48 0x9b4c 0x9b50 0x9b54 0x9b58 0x9b5c 0x9b60 0x9b64 0x9bac 0x9bb0 0x9bb4 0x9bb8 0x9bbc 0x9bc0 0x9bc4 0x9bc8 0x9bcc 0x9bd0 0x9bd4 0x9bd8 0x9bdc 0x9be0 0x9be4 0x9be8 0x9bec 0x9bf0 0x9bf4 0x9bf8 0x9bfc 0x9c00 0x9c04 0x9c08 0x9c0c 0x9c10 0x9c14 0x9c18 0x9c1c 0x9c20 0x9c24 0x9c28 0x9c2c 0x9c6c 0x9c70 0x9c74 0x9c78 0x9c7c 0x9c80 0x9c84 0x9c88 0x9c8c 0x9c90 0x9c94 0x9c98 0x9c9c 0x9ca0 0x9ca4 0x9ca8 0x9cac 0x9cb0 0x9cb4 0x9cb8 0x9d00 0x9d04 0x9d08 0x9d0c 0x9d10 0x9d14 0x9d18 0x9d1c 0x9d20 0x9d24 0x9d28 0x9d2c 0x9d30 0x9d34 0x9d38 0x9d3c 0x9d40 0x9d44 0x9d48 0x9d4c 0x9d50 0x9d54 0x9d58 0x9d5c 0x9d60 0x9d64 0x9d68 0x9d6c 0x9d70 0x9d74 0x9d78 0x9d7c 0x9d80 0x9d84 0x9d88 0x9d8c 0x9d90 0x9d94 0x9d98 0x9d9c 0x9da0 0x9da4 0x9da8 0x9dac 0x9db0 0x9db4 0x9db8 0x9dbc 0x9dc0 0x9dc4 0x9e0c 0x9e10 0x9e14 0x9e18 0x9e1c 0x9e20 0x9e24 0x9e28 0x9e2c 0x9e30 0x9e34 0x9e38 0x9e3c 0x9e40 0x9e44 0x9e48 0x9e4c 0x9e50 0x9e54 0x9e58 0x9e5c 0x9e60 0x9e64 0x9e68 0x9e6c 0x9e70 0x9e74 0x9e78 0x9e7c 0x9e80 0x9e84 0x9e88 0x9e8c 0x9e90 0x9e94 0x9e98 0x9e9c 0x9ea0 0x9ea4 0x9ea8 0x9eac 0x9eb0 0x9eb4 0x9eb8 0x9ebc 0x9ec0 0x9ec4 0x9ec8 0x9ecc 0x9ed0 0x9ed4 0x9ed8 0x9edc 0x9ee0 0x9ee4 0x9ee8 0x9eec 0x9ef0 0x9ef4 0x9ef8 0x9efc 0x9f00 0x9f04 0x9f08 0x9f0c 0x9f10 0x9f14 0x9f18 0x9f1c 0x9f20 0x9f24 0x9f28 0x9f2c 0x9f30 0x9f34 0x9f38 0x9f3c 0x9f40 0x9f44 0x9f48 0x9f4c 0x9f50 0x9f54 0x9f58 0x9f5c 0x9f60 0x9f64 0x9f68 0x9f6c 0x9f70 0x9f74 0x9f78 0x9f7c 0x9f80 0x9f84 0x9f88 0x9f8c 0x9f90 0x9f94 0x9f98 0x9f9c 0x9fa0 0x9fa4 0x9fa8 0x9fac 0x9fb0 0x9fb4 0x9fb8 0x9fbc 0x9fc0 0x9fc4 0x9fc8 0x9fcc 0x9fd0 0x9fd4 0x9fd8 0x9fdc 0x9fe0 0x9fe4 0x9fe8 0x9fec 0x9ff0 0x9ff4 0x9ff8 0x9ffc 0xa000 0xa004 0xa008 0xa00c 0xa010 0xa014 0xa018 0xa01c 0xa020 0xa024 0xa028 0xa02c 0xa030 0xa034 0xa038 0xa03c 0xa040 0xa044 0xa048 0xa04c 0xa050 0xa054 0xa058 0xa05c 0xa060 0xa064 0xa068 0xa06c 0xa070 0xa074 0xa078 0xa07c 0xa080 0xa084 0xa088 0xa08c 0xa090 0xa094 0xa098 0xa09c 0xa0a0 0xa0a4 0xa0a8 0xa0ac 0xa0b0 0xa0b4 0xa0b8 0xa0bc 0xa0c0 0xa0c4 0xa0c8 0xa0cc 0xa0d0 0xa0d4 0xa0d8 0xa0dc 0xa0e0 0xa0e4 0xa0e8 0xa0ec 0xa0f0 0xa0f4 0xa0f8 0xa0fc 0xa100 0xa104 0xa108 0xa10c 0xa110 0xa114 0xa118 0xa11c 0xa120 0xa124 0xa128 0xa12c 0xa130 0xa134 0xa138 0xa13c 0xa140 0xa144 0xa148 0xa14c 0xa150 0xa154 0xa158 0xa15c 0xa160 0xa164 0xa168 0xa1b8 0xa1bc 0xa1c0 0xa1c4 0xa1c8 0xa1cc 0xa1d0 0xa1d4 0xa1d8 0xa1dc 0xa1e0 0xa1e4 0xa1e8 0xa1ec 0xa1f0 0xa1f4 0xa1f8 0xa1fc 0xa200 0xa204 0xa208 0xa20c 0xa210 0xa214 0xa218 0xa21c 0xa220 0xa224 0xa228 0xa22c 0xa230 0xa234 0xa238 0xa23c 0xa240 0xa244 0xa248 0xa24c 0xa250 0xa254 0xa258 0xa25c 0xa260 0xa264 0xa268 0xa26c 0xa270 0xa274 0xa278 0xa27c 0xa280 0xa284 0xa288 0xa28c 0xa290 0xa294 0xa298 0xa29c 0xa2a0 0xa2a4 0xa2a8 0xa2ac 0xa2b0 0xa2b4 0xa2b8 0xa2bc 0xa2c0 0xa2c4 0xa2c8 0xa2cc 0xa2d0 0xa2d4 0xa2d8 0xa2dc 0xa2e0 0xa2e4 0xa2e8 0xa2ec 0xa2f0 0xa2f4 0xa348 0xa34c 0xa350 0xa354 0xa358 0xa35c 0xa360 0xa364 0xa368 0xa36c 0xa370 0xa374 0xa378 0xa37c 0xa380 0xa384 0xa388 0xa38c 0xa390 0xa394 0xa398 0xa39c 0xa3a0 0xa3a4 0xa3a8 0xa3ac 0xa3b0 0xa3b4 0xa3b8 0xa3bc 0xa3c0 0xa3c4 0xa3c8 0xa3cc 0xa3d0 0xa3d4 0xa3d8 0xa3dc 0xa3e0 0xa3e4 0xa3e8 0xa3ec 0xa3f0 0xa3f4 0xa3f8 0xa3fc 0xa400 0xa404 0xa408 0xa40c 0xa410 0xa414 0xa418 0xa41c 0xa420 0xa424 0xa428 0xa42c 0xa430 0xa434 0xa438 0xa43c 0xa440 0xa444 0xa448 0xa44c 0xa450 0xa454 0xa458 0xa45c 0xa460 0xa464 0xa468 0xa46c 0xa470 0xa474 0xa478 0xa47c 0xa480 0xa484 0xa488 0xa48c 0xa490 0xa494 0xa498 0xa49c 0xa4a0 0xa4a4 0xa4a8 0xa4ac 0xa4b0 0xa4b4 0xa4b8 0xa4bc 0xa4c0 0xa4c4 0xa4c8 0xa4cc 0xa4d0 0xa4d4 0xa4dc 0xa4e0 0xa4e4 0xa4e8 0xa4ec 0xa4f0 0xa4f4 0xa4f8 0xa4fc 0xa500 0xa504 0xa508 0xa50c 0xa510 0xa514 0xa518 0xa51c 0xa520 0xa524 0xa528 0xa52c 0xa530 0xa534 0xa538 0xa53c 0xa540 0xa544 0xa548 0xa54c 0xa550 0xa554 0xa558 0xa598 0xa59c 0xa5a0 0xa5a4 0xa5a8 0xa5ac 0xa5b0 0xa5b4 0xa5b8 0xa5bc 0xa5c0 0xa5c4 0xa5c8 0xa5cc 0xa5d0 0xa5d4 0xa5d8'
