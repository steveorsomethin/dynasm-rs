Ops!(

"aaa" = [
    &[]                                                                              ,  [0x37              ], X, X86_ONLY;
]
"aad" = [
    &[]                                                                              ,  [0xD5, 0x0A        ], X, X86_ONLY;
]
"aam" = [
    &[]                                                                              ,  [0xD4, 0x0A        ], X, X86_ONLY;
]
"aas" = [
    &[]                                                                              ,  [0x3F              ], X, X86_ONLY;
]
"adc" = [
    &[rax, Byte, Immediate, Byte]                                                    ,  [0x14              ], X;
    &[Memory, Byte, Immediate, Byte]                                                 ,  [0x80              ], 2, LOCK;
    &[Memory, Byte, Legacy, Byte]                                                    ,  [0x10              ], X, LOCK | ENC_MR;
    &[Legacy, Byte, Immediate, Byte]                                                 ,  [0x80              ], 2;
    &[Legacy, Byte, Legacy, Byte]                                                    ,  [0x10              ], X, ENC_MR;
    &[Legacy, Byte, LegacyMemory, Byte]                                              ,  [0x12              ], X;
    &[Legacy, Auto, Immediate, Byte]                                                 ,  [0x83              ], 2, AUTO_SIZE  | EXACT_SIZE;
    &[rax, Auto, Immediate, Auto]                                                    ,  [0x15              ], X, AUTO_SIZE;
    &[Memory, Auto, Immediate, Auto]                                                 ,  [0x81              ], 2, AUTO_SIZE | LOCK;
    &[Memory, Auto, Immediate, Byte]                                                 ,  [0x83              ], 2, AUTO_SIZE | LOCK;
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x11              ], X, AUTO_SIZE | LOCK | ENC_MR;
    &[Legacy, Auto, Immediate, Auto]                                                 ,  [0x81              ], 2, AUTO_SIZE ;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x11              ], X, AUTO_SIZE | ENC_MR;
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x13              ], X, AUTO_SIZE;
]
"adcx" = [
    &[Legacy, QWord, LegacyMemory, QWord]                                            ,  [0x0F, 0x38, 0xF6  ], X, WITH_REXW | PREF_66;
]
"add" = [
    &[rax, Byte, Immediate, Byte]                                                    ,  [0x04              ], X;
    &[Memory, Byte, Immediate, Byte]                                                 ,  [0x80              ], 0, LOCK;
    &[Memory, Byte, Legacy, Byte]                                                    ,  [0x00              ], X, LOCK | ENC_MR;
    &[Legacy, Byte, Immediate, Byte]                                                 ,  [0x80              ], 0;
    &[Legacy, Byte, Legacy, Byte]                                                    ,  [0x00              ], X, ENC_MR;
    &[Legacy, Byte, LegacyMemory, Byte]                                              ,  [0x02              ], X;
    &[Legacy, Auto, Immediate, Byte]                                                 ,  [0x83              ], 0, AUTO_SIZE  | EXACT_SIZE;
    &[rax, Auto, Immediate, Auto]                                                    ,  [0x05              ], X, AUTO_SIZE;
    &[Memory, Auto, Immediate, Auto]                                                 ,  [0x81              ], 0, AUTO_SIZE | LOCK;
    &[Memory, Auto, Immediate, Byte]                                                 ,  [0x83              ], 0, AUTO_SIZE | LOCK;
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x01              ], X, AUTO_SIZE | LOCK | ENC_MR;
    &[Legacy, Auto, Immediate, Auto]                                                 ,  [0x81              ], 0, AUTO_SIZE ;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x01              ], X, AUTO_SIZE | ENC_MR;
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x03              ], X, AUTO_SIZE;
]
"addpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x58        ], X, PREF_66, SSE2;
]
"addps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x58        ], X, DEFAULT, SSE;
]
"addsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x58        ], X, PREF_F2, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x58        ], X, PREF_F2, SSE2;
]
"addss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x58        ], X, PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x58        ], X, PREF_F3, SSE;
]
"addsubpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xD0        ], X, PREF_66, SSE3;
]
"addsubps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xD0        ], X, PREF_F2, SSE3;
]
"adox" = [
    &[Legacy, QWord, LegacyMemory, QWord]                                            ,  [0x0F, 0x38, 0xF6  ], X, WITH_REXW | PREF_F3;
]
"aesdec" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x38, 0xDE  ], X, PREF_66, SSE;
]
"aesdeclast" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x38, 0xDF  ], X, PREF_66, SSE;
]
"aesenc" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x38, 0xDC  ], X, PREF_66, SSE;
]
"aesenclast" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x38, 0xDD  ], X, PREF_66, SSE;
]
"aesimc" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x38, 0xDB  ], X, PREF_66, SSE;
]
"aeskeygenassist" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x0F, 0x3A, 0xDF  ], X, PREF_66, SSE;
]
"and" = [
    &[rax, Byte, Immediate, Byte]                                                    ,  [0x24              ], X;
    &[Memory, Byte, Immediate, Byte]                                                 ,  [0x80              ], 4, LOCK;
    &[Memory, Byte, Legacy, Byte]                                                    ,  [0x20              ], X, LOCK | ENC_MR;
    &[Legacy, Byte, Immediate, Byte]                                                 ,  [0x80              ], 4;
    &[Legacy, Byte, Legacy, Byte]                                                    ,  [0x20              ], X, ENC_MR;
    &[Legacy, Byte, LegacyMemory, Byte]                                              ,  [0x22              ], X;
    &[Legacy, Auto, Immediate, Byte]                                                 ,  [0x83              ], 4, AUTO_SIZE  | EXACT_SIZE;
    &[rax, Auto, Immediate, Auto]                                                    ,  [0x25              ], X, AUTO_SIZE;
    &[Memory, Auto, Immediate, Auto]                                                 ,  [0x81              ], 4, AUTO_SIZE | LOCK;
    &[Memory, Auto, Immediate, Byte]                                                 ,  [0x83              ], 4, AUTO_SIZE | LOCK;
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x21              ], X, AUTO_SIZE | LOCK | ENC_MR;
    &[Legacy, Auto, Immediate, Auto]                                                 ,  [0x81              ], 4, AUTO_SIZE ;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x21              ], X, AUTO_SIZE | ENC_MR;
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x23              ], X, AUTO_SIZE;
]
"andn" = [
    &[Legacy, Auto, Legacy, Auto, LegacyMemory, Auto]                                ,  [0x02, 0xF2        ], X, VEX_OP | AUTO_REXW, BMI1;
]
"andnpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x55        ], X, PREF_66, SSE2;
]
"andnps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x55        ], X, DEFAULT, SSE;
]
"andpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x54        ], X, PREF_66, SSE2;
]
"andps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x54        ], X, DEFAULT, SSE;
]
"arpl" = [
    &[LegacyMemory, Word, Legacy, Word]                                              ,  [0x63              ], X, X86_ONLY;
]
"bextr" = [
    &[Legacy, Auto, LegacyMemory, Auto, Immediate, DWord]                            ,  [0x10, 0x10        ], X, XOP_OP | AUTO_REXW, TBM;
    &[Legacy, Auto, LegacyMemory, Auto, Legacy, Auto]                                ,  [0x02, 0xF7        ], X, VEX_OP | AUTO_REXW | ENC_MR, BMI1;
]
"blcfill" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x09, 0x01        ], 1, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blci" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x09, 0x02        ], 6, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blcic" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x09, 0x01        ], 5, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blcmsk" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x09, 0x02        ], 1, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blcs" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x09, 0x01        ], 3, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blendpd" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x0D  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x0D  ], X, PREF_66, SSE41;
]
"blendps" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x0C  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x0C  ], X, PREF_66, SSE41;
]
"blendvpd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x15  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x15  ], X, PREF_66, SSE41;
]
"blendvps" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x14  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x14  ], X, PREF_66, SSE41;
]
"blsfill" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x09, 0x01        ], 2, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blsi" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x02, 0xF3        ], 3, VEX_OP | AUTO_REXW | ENC_VM, BMI1;
]
"blsic" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x09, 0x01        ], 6, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blsmsk" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x02, 0xF3        ], 2, VEX_OP | AUTO_REXW | ENC_VM, BMI1;
]
"blsr" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x02, 0xF3        ], 1, VEX_OP | AUTO_REXW | ENC_VM, BMI1;
]
"bound" = [
    &[Legacy, Auto, Memory, MemAuto]                                                 ,  [0x62              ], X, AUTO_SIZE | X86_ONLY;
]
"bndcl" = [
    &[Byte, OWord, Memory, MemAuto]                                                  ,  [0x0F, 0x1A        ], X, PREF_F3, MPX;
    &[Byte, OWord, Legacy, QWord]                                                    ,  [0x0F, 0x1A        ], X,  PREF_F3, MPX;
]
"bndcn" = [
    &[Byte, OWord, Memory, MemAuto]                                                  ,  [0x0F, 0x1B        ], X, PREF_F2, MPX;
    &[Byte, OWord, Legacy, QWord]                                                    ,  [0x0F, 0x1B        ], X,  PREF_F2, MPX;
]
"bndcu" = [
    &[Byte, OWord, Memory, MemAuto]                                                  ,  [0x0F, 0x1A        ], X, PREF_F2, MPX;
    &[Byte, OWord, Legacy, QWord]                                                    ,  [0x0F, 0x1A        ], X,  PREF_F2, MPX;
]
"bndldx" = [
    &[Byte, OWord, Memory, MemAuto]                                                  ,  [0x0F, 0x1A        ], X, ENC_MIB, MPX;
]
"bndmk" = [
    &[Byte, OWord, Memory, MemAuto]                                                  ,  [0x0F, 0x1B        ], X, ENC_MIB | PREF_F3, MPX;
]
"bndmov" = [
    &[Byte, OWord, Byte, OWord]                                                      ,  [0x0F, 0x1A        ], X, PREF_66, MPX;
    &[Byte, OWord, Byte, OWord]                                                      ,  [0x0F, 0x1B        ], X, ENC_MR | PREF_66, MPX;
    &[Byte, OWord, Memory, MemAuto]                                                  ,  [0x0F, 0x1A        ], X, PREF_66, MPX;
    &[Memory, MemAuto, Byte, OWord]                                                  ,  [0x0F, 0x1B        ], X, ENC_MR | PREF_66, MPX;
]
"bndstx" = [
    &[Memory, MemAuto, Byte, OWord]                                                  ,  [0x0F, 0x1B        ], X, ENC_MR | ENC_MIB, MPX;
]
"bsf" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x0F, 0xBC        ], X, AUTO_SIZE;
]
"bsr" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x0F, 0xBD        ], X, AUTO_SIZE;
]
"bswap" = [
    &[Legacy, Auto]                                                                  ,  [0x0F, 0xC8        ], X, AUTO_REXW | SHORT_ARG;
]
"bt" = [
    &[LegacyMemory, Auto, Immediate, Byte]                                           ,  [0x0F, 0xBA        ], 4, AUTO_SIZE;
    &[LegacyMemory, Auto, Legacy, Auto]                                              ,  [0x0F, 0xA3        ], X, AUTO_SIZE | ENC_MR;
]
"btc" = [
    &[Legacy, Auto, Immediate, Byte]                                                 ,  [0x0F, 0xBA        ], 7, AUTO_SIZE  | EXACT_SIZE;
    &[Memory, Auto, Immediate, Byte]                                                 ,  [0x0F, 0xBA        ], 7, AUTO_SIZE | LOCK;
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x0F, 0xBB        ], X, AUTO_SIZE | LOCK | ENC_MR;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x0F, 0xBB        ], X, AUTO_SIZE | ENC_MR;
]
"btr" = [
    &[Legacy, Auto, Immediate, Byte]                                                 ,  [0x0F, 0xBA        ], 6, AUTO_SIZE  | EXACT_SIZE;
    &[Memory, Auto, Immediate, Byte]                                                 ,  [0x0F, 0xBA        ], 6, AUTO_SIZE | LOCK;
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x0F, 0xB3        ], X, AUTO_SIZE | LOCK | ENC_MR;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x0F, 0xB3        ], X, AUTO_SIZE | ENC_MR;
]
"bts" = [
    &[Legacy, Auto, Immediate, Byte]                                                 ,  [0x0F, 0xBA        ], 5, AUTO_SIZE  | EXACT_SIZE;
    &[Memory, Auto, Immediate, Byte]                                                 ,  [0x0F, 0xBA        ], 5, AUTO_SIZE | LOCK;
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x0F, 0xAB        ], X, AUTO_SIZE | LOCK | ENC_MR;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x0F, 0xAB        ], X, AUTO_SIZE | ENC_MR;
]
"bzhi" = [
    &[Legacy, Auto, LegacyMemory, Auto, Legacy, Auto]                                ,  [0x02, 0xF5        ], X, VEX_OP | AUTO_REXW | ENC_MR, BMI2;
]
"cbw" = [
    &[]                                                                              ,  [0x98              ], X, WORD_SIZE;
]
"cdq" = [
    &[]                                                                              ,  [0x99              ], X;
]
"cdqe" = [
    &[]                                                                              ,  [0x98              ], X, WITH_REXW;
]
"clac" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xCA  ], X;
]
"clc" = [
    &[]                                                                              ,  [0xF8              ], X;
]
"cld" = [
    &[]                                                                              ,  [0xFC              ], X;
]
"clflush" = [
    &[Memory, Byte]                                                                  ,  [0x0F, 0xAE        ], 7, DEFAULT, SSE2;
]
"clgi" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xDD  ], X, DEFAULT, VMX | AMD;
]
"cli" = [
    &[]                                                                              ,  [0xFA              ], X;
]
"clts" = [
    &[]                                                                              ,  [0x0F, 0x06        ], X;
]
"clzero" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xFC  ], X, DEFAULT, AMD;
]
"cmc" = [
    &[]                                                                              ,  [0xF5              ], X;
]
"cmp" = [
    &[rax, Byte, Immediate, Byte]                                                    ,  [0x3C              ], X;
    &[Legacy, Byte, LegacyMemory, Byte]                                              ,  [0x3A              ], X;
    &[LegacyMemory, Byte, Immediate, Byte]                                           ,  [0x80              ], 7;
    &[LegacyMemory, Byte, Legacy, Byte]                                              ,  [0x38              ], X, ENC_MR;
    &[rax, Auto, Immediate, Auto]                                                    ,  [0x3D              ], X, AUTO_SIZE;
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x3B              ], X, AUTO_SIZE;
    &[LegacyMemory, Auto, Immediate, Auto]                                           ,  [0x81              ], 7, AUTO_SIZE;
    &[LegacyMemory, Auto, Immediate, Byte]                                           ,  [0x83              ], 7, AUTO_SIZE;
    &[LegacyMemory, Auto, Legacy, Auto]                                              ,  [0x39              ], X, AUTO_SIZE | ENC_MR;
]
"cmpeqpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x00  ], X, PREF_66 | IMM_OP, SSE2;
]
"cmpeqps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x00  ], X, IMM_OP, SSE;
]
"cmpeqsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0xC2, 0x00  ], X, PREF_F2 | IMM_OP, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x00  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpeqss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0xC2, 0x00  ], X, PREF_F3 | IMM_OP, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x00  ], X, PREF_F3 | IMM_OP, SSE;
]
"cmplepd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x02  ], X, IMM_OP | PREF_66, SSE2;
]
"cmpleps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x02  ], X, IMM_OP, SSE;
]
"cmplesd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0xC2, 0x02  ], X, PREF_F2 | IMM_OP, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x02  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpless" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0xC2, 0x02  ], X, PREF_F3 | IMM_OP, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x02  ], X, PREF_F3 | IMM_OP, SSE;
]
"cmpltpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x01  ], X, IMM_OP | PREF_66, SSE2;
]
"cmpltps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x01  ], X, IMM_OP, SSE;
]
"cmpltsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0xC2, 0x01  ], X, PREF_F2 | IMM_OP, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x01  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpltss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0xC2, 0x01  ], X, IMM_OP | PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x01  ], X, IMM_OP | PREF_F3, SSE;
]
"cmpneqpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x04  ], X, PREF_66 | IMM_OP, SSE2;
]
"cmpneqps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x04  ], X, IMM_OP, SSE;
]
"cmpneqsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0xC2, 0x04  ], X, PREF_F2 | IMM_OP, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x04  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpneqss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0xC2, 0x04  ], X, IMM_OP | PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x04  ], X, IMM_OP | PREF_F3, SSE;
]
"cmpnlepd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x06  ], X, IMM_OP | PREF_66, SSE2;
]
"cmpnleps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x06  ], X, IMM_OP, SSE;
]
"cmpnlesd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0xC2, 0x06  ], X, PREF_F2 | IMM_OP, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x06  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpnless" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0xC2, 0x06  ], X, IMM_OP | PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x06  ], X, IMM_OP | PREF_F3, SSE;
]
"cmpnltpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x05  ], X, PREF_66 | IMM_OP, SSE2;
]
"cmpnltps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x05  ], X, IMM_OP, SSE;
]
"cmpnltsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0xC2, 0x05  ], X, IMM_OP | PREF_F2, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x05  ], X, IMM_OP | PREF_F2, SSE2;
]
"cmpnltss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0xC2, 0x05  ], X, PREF_F3 | IMM_OP, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x05  ], X, PREF_F3 | IMM_OP, SSE;
]
"cmpordpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x07  ], X, IMM_OP | PREF_66, SSE2;
]
"cmpordps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x07  ], X, IMM_OP, SSE;
]
"cmpordsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0xC2, 0x07  ], X, PREF_F2 | IMM_OP, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x07  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpordss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0xC2, 0x07  ], X, IMM_OP | PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x07  ], X, IMM_OP | PREF_F3, SSE;
]
"cmppd" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x0F, 0xC2        ], X, PREF_66, SSE2;
]
"cmpps" = [
    &[AVXRegister, OWord, Memory, MemAuto, Immediate, Byte]                          ,  [0x0F, 0xC2        ], X, DEFAULT, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0xC2        ], X, DEFAULT, SSE;
]
"cmpsb" = [
    &[]                                                                              ,  [0xA6              ], X, REPE;
]
"cmpsd" = [
    &[]                                                                              ,  [0xA7              ], X, REPE;
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x0F, 0xC2        ], X, PREF_F2, SSE2;
]
"cmpsq" = [
    &[]                                                                              ,  [0xA7              ], X, REPE | WITH_REXW;
]
"cmpss" = [
    &[AVXRegister, OWord, Memory, MemAuto, Immediate, Byte]                          ,  [0x0F, 0xC2        ], X, PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0xC2        ], X, PREF_F3, SSE;
]
"cmpsw" = [
    &[]                                                                              ,  [0xA7              ], X, REPE | WORD_SIZE;
]
"cmpunordpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x03  ], X, PREF_66 | IMM_OP, SSE2;
]
"cmpunordps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xC2, 0x03  ], X, IMM_OP, SSE;
]
"cmpunordsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0xC2, 0x03  ], X, PREF_F2 | IMM_OP, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x03  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpunordss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0xC2, 0x03  ], X, PREF_F3 | IMM_OP, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xC2, 0x03  ], X, PREF_F3 | IMM_OP, SSE;
]
"cmpxchg" = [
    &[Memory, Byte, Legacy, Byte]                                                    ,  [0x0F, 0xB0        ], X, LOCK | ENC_MR;
    &[Legacy, Byte, Legacy, Byte]                                                    ,  [0x0F, 0xB0        ], X, ENC_MR;
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x0F, 0xB1        ], X, AUTO_SIZE | LOCK | ENC_MR;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x0F, 0xB1        ], X, AUTO_SIZE | ENC_MR;
]
"cmpxchg16b" = [
    &[Memory, OWord]                                                                 ,  [0x0F, 0xC7        ], 1, LOCK | WITH_REXW;
]
"cmpxchg8b" = [
    &[Memory, QWord]                                                                 ,  [0x0F, 0xC7        ], 1, LOCK;
]
"comisd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x2F        ], X, PREF_66, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x2F        ], X, PREF_66, SSE2;
]
"comiss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x2F        ], X, DEFAULT, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x2F        ], X, DEFAULT, SSE;
]
"cpu_read" = [
    &[]                                                                              ,  [0x0F, 0x3D        ], X, DEFAULT, CYRIX;
]
"cpu_write" = [
    &[]                                                                              ,  [0x0F, 0x3C        ], X, DEFAULT, CYRIX;
]
"cpuid" = [
    &[]                                                                              ,  [0x0F, 0xA2        ], X;
]
"cqo" = [
    &[]                                                                              ,  [0x99              ], X, WITH_REXW;
]
"cvtdq2pd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0xE6        ], X, PREF_F3, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xE6        ], X, PREF_F3, SSE2;
]
"cvtdq2ps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x5B        ], X, DEFAULT, SSE2;
]
"cvtpd2dq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xE6        ], X, PREF_F2, SSE2;
]
"cvtpd2pi" = [
    &[MMXRegister, QWord, Word, OWord]                                               ,  [0x0F, 0x2D        ], X, PREF_66, SSE2;
]
"cvtpd2ps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x5A        ], X, PREF_66, SSE2;
]
"cvtpi2pd" = [
    &[AVXRegister, OWord, MMXMemory, QWord]                                          ,  [0x0F, 0x2A        ], X, PREF_66, SSE2;
]
"cvtpi2ps" = [
    &[AVXRegister, OWord, MMXMemory, QWord]                                          ,  [0x0F, 0x2A        ], X, DEFAULT, MMX | SSE;
]
"cvtps2dq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x5B        ], X, PREF_66, SSE2;
]
"cvtps2pd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x5A        ], X, DEFAULT, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x5A        ], X, DEFAULT, SSE2;
]
"cvtps2pi" = [
    &[MMXRegister, QWord, Memory, QWord]                                             ,  [0x0F, 0x2D        ], X, DEFAULT, SSE | MMX;
    &[MMXRegister, QWord, AVXRegister, OWord]                                        ,  [0x0F, 0x2D        ], X, DEFAULT, SSE | MMX;
]
"cvtsd2si" = [
    &[Legacy, DWord, Memory, QWord]                                                  ,  [0x0F, 0x2D        ], X, PREF_F2, SSE2;
    &[Legacy, DWord, AVXRegister, OWord]                                             ,  [0x0F, 0x2D        ], X, PREF_F2, SSE2;
    &[Legacy, QWord, Memory, QWord]                                                  ,  [0x0F, 0x2D        ], X, WITH_REXW | PREF_F2, SSE2;
    &[Legacy, QWord, AVXRegister, OWord]                                             ,  [0x0F, 0x2D        ], X, WITH_REXW | PREF_F2, SSE2;
]
"cvtsd2ss" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x5A        ], X, PREF_F2, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x5A        ], X, PREF_F2, SSE2;
]
"cvtsi2sd" = [
    &[AVXRegister, OWord, LegacyMemory, DWord]                                       ,  [0x0F, 0x2A        ], X, PREF_F2, SSE2;
    &[AVXRegister, OWord, LegacyMemory, QWord]                                       ,  [0x0F, 0x2A        ], X, WITH_REXW | PREF_F2, SSE2;
]
"cvtsi2ss" = [
    &[AVXRegister, OWord, LegacyMemory, DWord]                                       ,  [0x0F, 0x2A        ], X, PREF_F3, SSE;
    &[AVXRegister, OWord, LegacyMemory, QWord]                                       ,  [0x0F, 0x2A        ], X, WITH_REXW | PREF_F3, SSE;
]
"cvtss2sd" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x5A        ], X, PREF_F3, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x5A        ], X, PREF_F3, SSE2;
]
"cvtss2si" = [
    &[Legacy, DWord, Memory, DWord]                                                  ,  [0x0F, 0x2D        ], X, PREF_F3, SSE;
    &[Legacy, DWord, AVXRegister, OWord]                                             ,  [0x0F, 0x2D        ], X, PREF_F3, SSE;
    &[Legacy, QWord, Memory, DWord]                                                  ,  [0x0F, 0x2D        ], X, WITH_REXW | PREF_F3, SSE;
    &[Legacy, QWord, AVXRegister, OWord]                                             ,  [0x0F, 0x2D        ], X, WITH_REXW | PREF_F3, SSE;
]
"cvttpd2dq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xE6        ], X, PREF_66, SSE2;
]
"cvttpd2pi" = [
    &[MMXRegister, QWord, Word, OWord]                                               ,  [0x0F, 0x2C        ], X, PREF_66, SSE2;
]
"cvttps2dq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x5B        ], X, PREF_F3, SSE2;
]
"cvttps2pi" = [
    &[MMXRegister, QWord, Memory, QWord]                                             ,  [0x0F, 0x2C        ], X, DEFAULT, SSE | MMX;
    &[MMXRegister, QWord, AVXRegister, OWord]                                        ,  [0x0F, 0x2C        ], X, DEFAULT, SSE | MMX;
]
"cvttsd2si" = [
    &[Legacy, DWord, Memory, QWord]                                                  ,  [0x0F, 0x2C        ], X, PREF_F2, SSE2;
    &[Legacy, DWord, AVXRegister, OWord]                                             ,  [0x0F, 0x2C        ], X, PREF_F2, SSE2;
    &[Legacy, QWord, Memory, QWord]                                                  ,  [0x0F, 0x2C        ], X, WITH_REXW | PREF_F2, SSE2;
    &[Legacy, QWord, AVXRegister, OWord]                                             ,  [0x0F, 0x2C        ], X, WITH_REXW | PREF_F2, SSE2;
]
"cvttss2si" = [
    &[Legacy, DWord, Memory, DWord]                                                  ,  [0x0F, 0x2C        ], X, PREF_F3, SSE;
    &[Legacy, DWord, AVXRegister, OWord]                                             ,  [0x0F, 0x2C        ], X, PREF_F3, SSE;
    &[Legacy, QWord, Memory, DWord]                                                  ,  [0x0F, 0x2C        ], X, WITH_REXW | PREF_F3, SSE;
    &[Legacy, QWord, AVXRegister, OWord]                                             ,  [0x0F, 0x2C        ], X, WITH_REXW | PREF_F3, SSE;
]
"cwd" = [
    &[]                                                                              ,  [0x99              ], X, WORD_SIZE;
]
"cwde" = [
    &[]                                                                              ,  [0x98              ], X;
]
"daa" = [
    &[]                                                                              ,  [0x27              ], X, X86_ONLY;
]
"das" = [
    &[]                                                                              ,  [0x2F              ], X, X86_ONLY;
]
"dec" = [
    &[Memory, Byte]                                                                  ,  [0xFE              ], 1, LOCK;
    &[Legacy, Byte]                                                                  ,  [0xFE              ], 1;
    &[Memory, Auto]                                                                  ,  [0xFF              ], 1, AUTO_SIZE | LOCK;
    &[Legacy, Auto]                                                                  ,  [0x48              ], 0, X86_ONLY | SHORT_ARG;
    &[Legacy, Auto]                                                                  ,  [0xFF              ], 1, AUTO_SIZE ;
]
"div" = [
    &[LegacyMemory, Byte]                                                            ,  [0xF6              ], 6;
    &[LegacyMemory, Auto]                                                            ,  [0xF7              ], 6, AUTO_SIZE;
]
"divpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x5E        ], X, PREF_66, SSE2;
]
"divps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x5E        ], X, DEFAULT, SSE;
]
"divsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x5E        ], X, PREF_F2, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x5E        ], X, PREF_F2, SSE2;
]
"divss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x5E        ], X, PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x5E        ], X, PREF_F3, SSE;
]
"dmint" = [
    &[]                                                                              ,  [0x0F, 0x39        ], X, DEFAULT, CYRIX;
]
"dppd" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x41  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x41  ], X, PREF_66, SSE41;
]
"dpps" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x40  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x40  ], X, PREF_66, SSE41;
]
"emms" = [
    &[]                                                                              ,  [0x0F, 0x77        ], X, DEFAULT, MMX;
]
"enter" = [
    &[Immediate, Word, Immediate, Byte]                                              ,  [0xC8              ], X;
]
"extractps" = [
    &[Legacy, QWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x17  ], X, WITH_REXW | ENC_MR | PREF_66, SSE41;
    &[LegacyMemory, DWord, AVXRegister, OWord, Immediate, Byte]                      ,  [0x0F, 0x3A, 0x17  ], X, ENC_MR | PREF_66, SSE41;
]
"extrq" = [
    &[AVXRegister, OWord, Immediate, Byte, Immediate, Byte]                          ,  [0x0F, 0x78        ], 0, PREF_66, SSE4A | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x79        ], X, PREF_66, SSE4A | AMD;
]
"f2xm1" = [
    &[]                                                                              ,  [0xD9, 0xF0        ], X, DEFAULT, FPU;
]
"fabs" = [
    &[]                                                                              ,  [0xD9, 0xE1        ], X, DEFAULT, FPU;
]
"fadd" = [
    &[]                                                                              ,  [0xDE, 0xC1        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xD8, 0xC0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xD8, 0xC0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDC, 0xC0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDC, 0xC0        ], X, SHORT_ARG, FPU;
    &[Memory, DWord]                                                                 ,  [0xD8              ], 0, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDC              ], 0, EXACT_SIZE, FPU;
]
"faddp" = [
    &[]                                                                              ,  [0xDE, 0xC1        ], X, DEFAULT, FPU;
    &[FWord, PWord]                                                                  ,  [0xDE, 0xC0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDE, 0xC0        ], X, SHORT_ARG, FPU;
]
"fbld" = [
    &[Memory, MemAuto]                                                               ,  [0xDF              ], 4, DEFAULT, FPU;
]
"fbstp" = [
    &[Memory, MemAuto]                                                               ,  [0xDF              ], 6, DEFAULT, FPU;
]
"fchs" = [
    &[]                                                                              ,  [0xD9, 0xE0        ], X, DEFAULT, FPU;
]
"fclex" = [
    &[]                                                                              ,  [0x9B, 0xDB, 0xE2  ], X, DEFAULT, FPU;
]
"fcmovb" = [
    &[]                                                                              ,  [0xDA, 0xC1        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDA, 0xC0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDA, 0xC0        ], X, SHORT_ARG, FPU;
]
"fcmovbe" = [
    &[]                                                                              ,  [0xDA, 0xD1        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDA, 0xD0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDA, 0xD0        ], X, SHORT_ARG, FPU;
]
"fcmove" = [
    &[]                                                                              ,  [0xDA, 0xC9        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDA, 0xC8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDA, 0xC8        ], X, SHORT_ARG, FPU;
]
"fcmovnb" = [
    &[]                                                                              ,  [0xDB, 0xC1        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDB, 0xC0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDB, 0xC0        ], X, SHORT_ARG, FPU;
]
"fcmovnbe" = [
    &[]                                                                              ,  [0xDB, 0xD1        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDB, 0xD0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDB, 0xD0        ], X, SHORT_ARG, FPU;
]
"fcmovne" = [
    &[]                                                                              ,  [0xDB, 0xC9        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDB, 0xC8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDB, 0xC8        ], X, SHORT_ARG, FPU;
]
"fcmovnu" = [
    &[]                                                                              ,  [0xDB, 0xD9        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDB, 0xD8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDB, 0xD8        ], X, SHORT_ARG, FPU;
]
"fcmovu" = [
    &[]                                                                              ,  [0xDA, 0xD9        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDA, 0xD8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDA, 0xD8        ], X, SHORT_ARG, FPU;
]
"fcom" = [
    &[]                                                                              ,  [0xD8, 0xD1        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xD8, 0xD0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xD8, 0xD0        ], X, SHORT_ARG, FPU;
    &[Memory, DWord]                                                                 ,  [0xD8              ], 2, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDC              ], 2, EXACT_SIZE, FPU;
]
"fcomi" = [
    &[]                                                                              ,  [0xDB, 0xF1        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDB, 0xF0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDB, 0xF0        ], X, SHORT_ARG, FPU;
]
"fcomip" = [
    &[]                                                                              ,  [0xDF, 0xF1        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDF, 0xF0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDF, 0xF0        ], X, SHORT_ARG, FPU;
]
"fcomp" = [
    &[]                                                                              ,  [0xD8, 0xD9        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xD8, 0xD8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xD8, 0xD8        ], X, SHORT_ARG, FPU;
    &[Memory, DWord]                                                                 ,  [0xD8              ], 3, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDC              ], 3, EXACT_SIZE, FPU;
]
"fcompp" = [
    &[]                                                                              ,  [0xDE, 0xD9        ], X, DEFAULT, FPU;
]
"fcos" = [
    &[]                                                                              ,  [0xD9, 0xFF        ], X, DEFAULT, FPU;
]
"fdecstp" = [
    &[]                                                                              ,  [0xD9, 0xF6        ], X, DEFAULT, FPU;
]
"fdisi" = [
    &[]                                                                              ,  [0x9B, 0xDB, 0xE1  ], X, DEFAULT, FPU;
]
"fdiv" = [
    &[]                                                                              ,  [0xDE, 0xF9        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xD8, 0xF0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xD8, 0xF0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDC, 0xF8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDC, 0xF8        ], X, SHORT_ARG, FPU;
    &[Memory, DWord]                                                                 ,  [0xD8              ], 6, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDC              ], 6, EXACT_SIZE, FPU;
]
"fdivp" = [
    &[]                                                                              ,  [0xDE, 0xF9        ], X, DEFAULT, FPU;
    &[FWord, PWord]                                                                  ,  [0xDE, 0xF8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDE, 0xF8        ], X, SHORT_ARG, FPU;
]
"fdivr" = [
    &[]                                                                              ,  [0xDE, 0xF1        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xD8, 0xF8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xD8, 0xF8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDC, 0xF0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDC, 0xF0        ], X, SHORT_ARG, FPU;
    &[Memory, DWord]                                                                 ,  [0xD8              ], 7, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDC              ], 7, EXACT_SIZE, FPU;
]
"fdivrp" = [
    &[]                                                                              ,  [0xDE, 0xF1        ], X, DEFAULT, FPU;
    &[FWord, PWord]                                                                  ,  [0xDE, 0xF0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDE, 0xF0        ], X, SHORT_ARG, FPU;
]
"femms" = [
    &[]                                                                              ,  [0x0F, 0x0E        ], X, DEFAULT, TDNOW;
]
"feni" = [
    &[]                                                                              ,  [0x9B, 0xDB, 0xE0  ], X, DEFAULT, FPU;
]
"ffree" = [
    &[]                                                                              ,  [0xDD, 0xC1        ], X, DEFAULT, FPU;
    &[FWord, PWord]                                                                  ,  [0xDD, 0xC0        ], X, SHORT_ARG, FPU;
]
"fiadd" = [
    &[Memory, DWord]                                                                 ,  [0xDA              ], 0, EXACT_SIZE, FPU;
    &[Memory, Word]                                                                  ,  [0xDE              ], 0, DEFAULT, FPU;
]
"ficom" = [
    &[Memory, DWord]                                                                 ,  [0xDA              ], 2, EXACT_SIZE, FPU;
    &[Memory, Word]                                                                  ,  [0xDE              ], 2, DEFAULT, FPU;
]
"ficomp" = [
    &[Memory, DWord]                                                                 ,  [0xDA              ], 3, EXACT_SIZE, FPU;
    &[Memory, Word]                                                                  ,  [0xDE              ], 3, DEFAULT, FPU;
]
"fidiv" = [
    &[Memory, DWord]                                                                 ,  [0xDA              ], 6, EXACT_SIZE, FPU;
    &[Memory, Word]                                                                  ,  [0xDE              ], 6, DEFAULT, FPU;
]
"fidivr" = [
    &[Memory, DWord]                                                                 ,  [0xDA              ], 7, EXACT_SIZE, FPU;
    &[Memory, Word]                                                                  ,  [0xDE              ], 7, DEFAULT, FPU;
]
"fild" = [
    &[Memory, DWord]                                                                 ,  [0xDB              ], 0, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDF              ], 5, EXACT_SIZE, FPU;
    &[Memory, Word]                                                                  ,  [0xDF              ], 0, DEFAULT, FPU;
]
"fimul" = [
    &[Memory, DWord]                                                                 ,  [0xDA              ], 1, EXACT_SIZE, FPU;
    &[Memory, Word]                                                                  ,  [0xDE              ], 1, DEFAULT, FPU;
]
"fincstp" = [
    &[]                                                                              ,  [0xD9, 0xF7        ], X, DEFAULT, FPU;
]
"finit" = [
    &[]                                                                              ,  [0x9B, 0xDB, 0xE3  ], X, DEFAULT, FPU;
]
"fist" = [
    &[Memory, DWord]                                                                 ,  [0xDB              ], 2, EXACT_SIZE, FPU;
    &[Memory, Word]                                                                  ,  [0xDF              ], 2, DEFAULT, FPU;
]
"fistp" = [
    &[Memory, DWord]                                                                 ,  [0xDB              ], 3, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDF              ], 7, EXACT_SIZE, FPU;
    &[Memory, Word]                                                                  ,  [0xDF              ], 3, DEFAULT, FPU;
]
"fisttp" = [
    &[Memory, DWord]                                                                 ,  [0xDB              ], 1, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDD              ], 1, EXACT_SIZE, FPU;
    &[Memory, Word]                                                                  ,  [0xDF              ], 1, DEFAULT, FPU;
]
"fisub" = [
    &[Memory, DWord]                                                                 ,  [0xDA              ], 4, EXACT_SIZE, FPU;
    &[Memory, Word]                                                                  ,  [0xDE              ], 4, DEFAULT, FPU;
]
"fisubr" = [
    &[Memory, DWord]                                                                 ,  [0xDA              ], 5, EXACT_SIZE, FPU;
    &[Memory, Word]                                                                  ,  [0xDE              ], 5, DEFAULT, FPU;
]
"fld" = [
    &[]                                                                              ,  [0xD9, 0xC1        ], X, DEFAULT, FPU;
    &[FWord, PWord]                                                                  ,  [0xD9, 0xC0        ], X, SHORT_ARG, FPU;
    &[Memory, DWord]                                                                 ,  [0xD9              ], 0, EXACT_SIZE, FPU;
    &[Memory, PWord]                                                                 ,  [0xDB              ], 5, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDD              ], 0, EXACT_SIZE, FPU;
]
"fld1" = [
    &[]                                                                              ,  [0xD9, 0xE8        ], X, DEFAULT, FPU;
]
"fldcw" = [
    &[Memory, Word]                                                                  ,  [0xD9              ], 5, DEFAULT, FPU;
]
"fldenv" = [
    &[Memory, MemAuto]                                                               ,  [0xD9              ], 4, DEFAULT, FPU;
]
"fldl2e" = [
    &[]                                                                              ,  [0xD9, 0xEA        ], X, DEFAULT, FPU;
]
"fldl2t" = [
    &[]                                                                              ,  [0xD9, 0xE9        ], X, DEFAULT, FPU;
]
"fldlg2" = [
    &[]                                                                              ,  [0xD9, 0xEC        ], X, DEFAULT, FPU;
]
"fldln2" = [
    &[]                                                                              ,  [0xD9, 0xED        ], X, DEFAULT, FPU;
]
"fldpi" = [
    &[]                                                                              ,  [0xD9, 0xEB        ], X, DEFAULT, FPU;
]
"fldz" = [
    &[]                                                                              ,  [0xD9, 0xEE        ], X, DEFAULT, FPU;
]
"fmul" = [
    &[]                                                                              ,  [0xDE, 0xC9        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xD8, 0xC8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xD8, 0xC8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDC, 0xC8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDC, 0xC8        ], X, SHORT_ARG, FPU;
    &[Memory, DWord]                                                                 ,  [0xD8              ], 1, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDC              ], 1, EXACT_SIZE, FPU;
]
"fmulp" = [
    &[]                                                                              ,  [0xDE, 0xC9        ], X, DEFAULT, FPU;
    &[FWord, PWord]                                                                  ,  [0xDE, 0xC8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDE, 0xC8        ], X, SHORT_ARG, FPU;
]
"fnclex" = [
    &[]                                                                              ,  [0xDB, 0xE2        ], X, DEFAULT, FPU;
]
"fndisi" = [
    &[]                                                                              ,  [0xDB, 0xE1        ], X, DEFAULT, FPU;
]
"fneni" = [
    &[]                                                                              ,  [0xDB, 0xE0        ], X, DEFAULT, FPU;
]
"fninit" = [
    &[]                                                                              ,  [0xDB, 0xE3        ], X, DEFAULT, FPU;
]
"fnop" = [
    &[]                                                                              ,  [0xD9, 0xD0        ], X, DEFAULT, FPU;
]
"fnsave" = [
    &[Memory, MemAuto]                                                               ,  [0xDD              ], 6, DEFAULT, FPU;
]
"fnstcw" = [
    &[Memory, Word]                                                                  ,  [0xD9              ], 7, DEFAULT, FPU;
]
"fnstenv" = [
    &[Memory, MemAuto]                                                               ,  [0xD9              ], 6, DEFAULT, FPU;
]
"fnstsw" = [
    &[rax, Word]                                                                     ,  [0xDF, 0xE0        ], X, DEFAULT, FPU;
    &[Memory, Word]                                                                  ,  [0xDD              ], 7, DEFAULT, FPU;
]
"fpatan" = [
    &[]                                                                              ,  [0xD9, 0xF3        ], X, DEFAULT, FPU;
]
"fprem" = [
    &[]                                                                              ,  [0xD9, 0xF8        ], X, DEFAULT, FPU;
]
"fprem1" = [
    &[]                                                                              ,  [0xD9, 0xF5        ], X, DEFAULT, FPU;
]
"fptan" = [
    &[]                                                                              ,  [0xD9, 0xF2        ], X, DEFAULT, FPU;
]
"frndint" = [
    &[]                                                                              ,  [0xD9, 0xFC        ], X, DEFAULT, FPU;
]
"frstor" = [
    &[Memory, MemAuto]                                                               ,  [0xDD              ], 4, DEFAULT, FPU;
]
"fsave" = [
    &[Memory, MemAuto]                                                               ,  [0x9B, 0xDD        ], 6, DEFAULT, FPU;
]
"fscale" = [
    &[]                                                                              ,  [0xD9, 0xFD        ], X, DEFAULT, FPU;
]
"fsetpm" = [
    &[]                                                                              ,  [0xDB, 0xE4        ], X, DEFAULT, FPU;
]
"fsin" = [
    &[]                                                                              ,  [0xD9, 0xFE        ], X, DEFAULT, FPU;
]
"fsincos" = [
    &[]                                                                              ,  [0xD9, 0xFB        ], X, DEFAULT, FPU;
]
"fsqrt" = [
    &[]                                                                              ,  [0xD9, 0xFA        ], X, DEFAULT, FPU;
]
"fst" = [
    &[]                                                                              ,  [0xDD, 0xD1        ], X, DEFAULT, FPU;
    &[FWord, PWord]                                                                  ,  [0xDD, 0xD0        ], X, SHORT_ARG, FPU;
    &[Memory, DWord]                                                                 ,  [0xD9              ], 2, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDD              ], 2, EXACT_SIZE, FPU;
]
"fstcw" = [
    &[Memory, Word]                                                                  ,  [0x9B, 0xD9        ], 7, DEFAULT, FPU;
]
"fstenv" = [
    &[Memory, MemAuto]                                                               ,  [0x9B, 0xD9        ], 6, DEFAULT, FPU;
]
"fstp" = [
    &[]                                                                              ,  [0xDD, 0xD9        ], X, DEFAULT, FPU;
    &[FWord, PWord]                                                                  ,  [0xDD, 0xD8        ], X, SHORT_ARG, FPU;
    &[Memory, DWord]                                                                 ,  [0xD9              ], 3, EXACT_SIZE, FPU;
    &[Memory, PWord]                                                                 ,  [0xDB              ], 7, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDD              ], 3, EXACT_SIZE, FPU;
]
"fstsw" = [
    &[rax, Word]                                                                     ,  [0x9B, 0xDF, 0xE0  ], X, DEFAULT, FPU;
    &[Memory, Word]                                                                  ,  [0x9B, 0xDD        ], 7, DEFAULT, FPU;
]
"fsub" = [
    &[]                                                                              ,  [0xDE, 0xE9        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xD8, 0xE0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xD8, 0xE0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDC, 0xE8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDC, 0xE8        ], X, SHORT_ARG, FPU;
    &[Memory, DWord]                                                                 ,  [0xD8              ], 4, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDC              ], 4, EXACT_SIZE, FPU;
]
"fsubp" = [
    &[]                                                                              ,  [0xDE, 0xE9        ], X, DEFAULT, FPU;
    &[FWord, PWord]                                                                  ,  [0xDE, 0xE8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDE, 0xE8        ], X, SHORT_ARG, FPU;
]
"fsubr" = [
    &[]                                                                              ,  [0xDE, 0xE1        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xD8, 0xE8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xD8, 0xE8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDC, 0xE0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDC, 0xE0        ], X, SHORT_ARG, FPU;
    &[Memory, DWord]                                                                 ,  [0xD8              ], 5, EXACT_SIZE, FPU;
    &[Memory, QWord]                                                                 ,  [0xDC              ], 5, EXACT_SIZE, FPU;
]
"fsubrp" = [
    &[]                                                                              ,  [0xDE, 0xE1        ], X, DEFAULT, FPU;
    &[FWord, PWord]                                                                  ,  [0xDE, 0xE0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xDE, 0xE0        ], X, SHORT_ARG, FPU;
]
"ftst" = [
    &[]                                                                              ,  [0xD9, 0xE4        ], X, DEFAULT, FPU;
]
"fucom" = [
    &[]                                                                              ,  [0xDD, 0xE1        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDD, 0xE0        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDD, 0xE0        ], X, SHORT_ARG, FPU;
]
"fucomi" = [
    &[]                                                                              ,  [0xDB, 0xE9        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDB, 0xE8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDB, 0xE8        ], X, SHORT_ARG, FPU;
]
"fucomip" = [
    &[]                                                                              ,  [0xDF, 0xE9        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDF, 0xE8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDF, 0xE8        ], X, SHORT_ARG, FPU;
]
"fucomp" = [
    &[]                                                                              ,  [0xDD, 0xE9        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xDD, 0xE8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xDD, 0xE8        ], X, SHORT_ARG, FPU;
]
"fucompp" = [
    &[]                                                                              ,  [0xDA, 0xE9        ], X, DEFAULT, FPU;
]
"fwait" = [
    &[]                                                                              ,  [0x9B              ], X;
]
"fxam" = [
    &[]                                                                              ,  [0xD9, 0xE5        ], X, DEFAULT, FPU;
]
"fxch" = [
    &[]                                                                              ,  [0xD9, 0xC9        ], X, DEFAULT, FPU;
    &[st0, PWord, FWord, PWord]                                                      ,  [0xD9, 0xC8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord]                                                                  ,  [0xD9, 0xC8        ], X, SHORT_ARG, FPU;
    &[FWord, PWord, st0, PWord]                                                      ,  [0xD9, 0xC8        ], X, SHORT_ARG, FPU;
]
"fxrstor" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xAE        ], 1, DEFAULT, SSE | FPU;
]
"fxrstor64" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xAE        ], 1, WITH_REXW, FPU | SSE;
]
"fxsave" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xAE        ], 0, DEFAULT, FPU | SSE;
]
"fxsave64" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xAE        ], 0, WITH_REXW, SSE | FPU;
]
"fxtract" = [
    &[]                                                                              ,  [0xD9, 0xF4        ], X, DEFAULT, FPU;
]
"fyl2x" = [
    &[]                                                                              ,  [0xD9, 0xF1        ], X, DEFAULT, FPU;
]
"fyl2xp1" = [
    &[]                                                                              ,  [0xD9, 0xF9        ], X, DEFAULT, FPU;
]
"getsec" = [
    &[]                                                                              ,  [0x0F, 0x37        ], X;
]
"haddpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x7C        ], X, PREF_66, SSE3;
]
"haddps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x7C        ], X, PREF_F2, SSE3;
]
"hlt" = [
    &[]                                                                              ,  [0xF4              ], X;
]
"hsubpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x7D        ], X, PREF_66, SSE3;
]
"hsubps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x7D        ], X, PREF_F2, SSE3;
]
"icebp" = [
    &[]                                                                              ,  [0xF1              ], X;
]
"idiv" = [
    &[LegacyMemory, Byte]                                                            ,  [0xF6              ], 7;
    &[LegacyMemory, Auto]                                                            ,  [0xF7              ], 7, AUTO_SIZE;
]
"inc" = [
    &[Memory, Byte]                                                                  ,  [0xFE              ], 0, LOCK;
    &[Legacy, Byte]                                                                  ,  [0xFE              ], 0;
    &[Memory, Auto]                                                                  ,  [0xFF              ], 0, AUTO_SIZE | LOCK;
    &[Legacy, Auto]                                                                  ,  [0x40              ], 0, X86_ONLY | SHORT_ARG;
    &[Legacy, Auto]                                                                  ,  [0xFF              ], 0, AUTO_SIZE ;
]
"insb" = [
    &[]                                                                              ,  [0x6C              ], X, REP;
]
"insd" = [
    &[]                                                                              ,  [0x6D              ], X, REP;
]
"insertps" = [
    &[AVXRegister, OWord, Memory, DWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x21  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x21  ], X, PREF_66, SSE41;
]
"insertq" = [
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x79        ], X, PREF_F2, SSE4A | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte, Immediate, Byte]      ,  [0x0F, 0x78        ], X, PREF_F2, AMD | SSE4A;
]
"insw" = [
    &[]                                                                              ,  [0x6D              ], X, WORD_SIZE | REP;
]
"int" = [
    &[Immediate, Byte]                                                               ,  [0xCD              ], X;
]
"into" = [
    &[]                                                                              ,  [0xCE              ], X, X86_ONLY;
]
"int01" = [
    &[]                                                                              ,  [0xF1              ], X;
]
"int03" = [
    &[]                                                                              ,  [0xCC              ], X;
]
"int1" = [
    &[]                                                                              ,  [0xF1              ], X;
]
"int3" = [
    &[]                                                                              ,  [0xCC              ], X;
]
"invd" = [
    &[]                                                                              ,  [0x0F, 0x08        ], X;
]
"invept" = [
    &[Legacy, QWord, Memory, OWord]                                                  ,  [0x0F, 0x38, 0x80  ], X, PREF_66, VMX;
]
"invlpg" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0x01        ], 7;
]
"invlpga" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xDF  ], X, DEFAULT, AMD;
    &[rax, QWord, rcx, DWord]                                                        ,  [0x0F, 0x01, 0xDF  ], X, DEFAULT, AMD;
]
"invpcid" = [
    &[Legacy, QWord, Memory, OWord]                                                  ,  [0x0F, 0x38, 0x82  ], X, PREF_66, INVPCID;
]
"invvpid" = [
    &[Legacy, QWord, Memory, OWord]                                                  ,  [0x0F, 0x38, 0x81  ], X, PREF_66, VMX;
]
"iret" = [
    &[]                                                                              ,  [0xCF              ], X;
]
"iretd" = [
    &[]                                                                              ,  [0xCF              ], X;
]
"iretq" = [
    &[]                                                                              ,  [0xCF              ], X, WITH_REXW;
]
"iretw" = [
    &[]                                                                              ,  [0xCF              ], X, WORD_SIZE;
]
"jecxz" = [
    &[OWord, Byte]                                                                   ,  [0xE3              ], X, PREF_67;
]
"jrcxz" = [
    &[OWord, Byte]                                                                   ,  [0xE3              ], X;
]
"lahf" = [
    &[]                                                                              ,  [0x9F              ], X;
]
"lar" = [
    &[Legacy, Auto, Memory, Word]                                                    ,  [0x0F, 0x02        ], X, AUTO_SIZE;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x0F, 0x02        ], X, AUTO_SIZE;
]
"lddqu" = [
    &[AVXRegister, OWord, Memory, OWord]                                             ,  [0x0F, 0xF0        ], X, PREF_F2, SSE3;
]
"ldmxcsr" = [
    &[Memory, DWord]                                                                 ,  [0x0F, 0xAE        ], 2, DEFAULT, SSE;
]
"lds" = [
    &[Legacy, Auto, Memory, MemAuto]                                                 ,  [0xC5              ], X, AUTO_SIZE | X86_ONLY;
]
"lea" = [
    &[Legacy, Auto, Memory, MemAuto]                                                 ,  [0x8D              ], X, AUTO_SIZE;
]
"leave" = [
    &[]                                                                              ,  [0xC9              ], X;
]
"les" = [
    &[Legacy, Auto, Memory, MemAuto]                                                 ,  [0xC4              ], X, AUTO_SIZE | X86_ONLY;
]
"lfence" = [
    &[]                                                                              ,  [0x0F, 0xAE, 0xE8  ], X, DEFAULT, AMD;
]
"lfs" = [
    &[Legacy, Auto, Memory, MemAuto]                                                 ,  [0x0F, 0xB4        ], X, AUTO_SIZE;
]
"lgdt" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0x01        ], 2;
]
"lgs" = [
    &[Legacy, Auto, Memory, MemAuto]                                                 ,  [0x0F, 0xB5        ], X, AUTO_SIZE;
]
"lidt" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0x01        ], 3;
]
"lldt" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0x00        ], 2;
    &[Legacy, Word]                                                                  ,  [0x0F, 0x00        ], 2;
]
"llwpcb" = [
    &[Legacy, Auto]                                                                  ,  [0x09, 0x12        ], 0, XOP_OP | AUTO_REXW, AMD;
]
"lmsw" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0x01        ], 6;
    &[Legacy, Word]                                                                  ,  [0x0F, 0x01        ], 6;
]
"lodsb" = [
    &[]                                                                              ,  [0xAC              ], X, REP;
]
"lodsd" = [
    &[]                                                                              ,  [0xAD              ], X, REP;
]
"lodsq" = [
    &[]                                                                              ,  [0xAD              ], X, WITH_REXW | REP;
]
"lodsw" = [
    &[]                                                                              ,  [0xAD              ], X, WORD_SIZE | REP;
]
"loop" = [
    &[OWord, Byte]                                                                   ,  [0xE2              ], X;
]
"loope" = [
    &[OWord, Byte]                                                                   ,  [0xE1              ], X;
]
"loopne" = [
    &[OWord, Byte]                                                                   ,  [0xE0              ], X;
]
"loopnz" = [
    &[OWord, Byte]                                                                   ,  [0xE0              ], X;
]
"loopz" = [
    &[OWord, Byte]                                                                   ,  [0xE1              ], X;
]
"lsl" = [
    &[Legacy, Auto, Memory, Word]                                                    ,  [0x0F, 0x03        ], X, AUTO_SIZE;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x0F, 0x03        ], X, AUTO_SIZE;
]
"lss" = [
    &[Legacy, Auto, Memory, MemAuto]                                                 ,  [0x0F, 0xB2        ], X, AUTO_SIZE;
]
"ltr" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0x00        ], 3;
    &[Legacy, Word]                                                                  ,  [0x0F, 0x00        ], 3;
]
"lwpins" = [
    &[Legacy, Auto, LegacyMemory, Auto, Immediate, DWord]                            ,  [0x10, 0x12        ], 0, XOP_OP | AUTO_REXW | ENC_VM, AMD;
]
"lwpval" = [
    &[Legacy, Auto, LegacyMemory, Auto, Immediate, DWord]                            ,  [0x10, 0x12        ], 1, XOP_OP | AUTO_REXW | ENC_VM, AMD;
]
"lzcnt" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x0F, 0xBD        ], X, AUTO_SIZE | PREF_F3, AMD;
]
"maskmovdqu" = [
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xF7        ], X, PREF_66, SSE2;
]
"maskmovq" = [
    &[MMXRegister, QWord, MMXRegister, QWord]                                        ,  [0x0F, 0xF7        ], X, DEFAULT, MMX;
]
"maxpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x5F        ], X, PREF_66, SSE2;
]
"maxps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x5F        ], X, DEFAULT, SSE;
]
"maxsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x5F        ], X, PREF_F2, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x5F        ], X, PREF_F2, SSE2;
]
"maxss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x5F        ], X, PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x5F        ], X, PREF_F3, SSE;
]
"mfence" = [
    &[]                                                                              ,  [0x0F, 0xAE, 0xF0  ], X, DEFAULT, AMD;
]
"minpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x5D        ], X, PREF_66, SSE2;
]
"minps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x5D        ], X, DEFAULT, SSE;
]
"minsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x5D        ], X, PREF_F2, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x5D        ], X, PREF_F2, SSE2;
]
"minss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x5D        ], X, PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x5D        ], X, PREF_F3, SSE;
]
"monitor" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xC8  ], X;
    &[rax, QWord, rcx, DWord, rdx, DWord]                                            ,  [0x0F, 0x01, 0xC8  ], X;
]
"monitorx" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xFA  ], X, DEFAULT, AMD;
    &[rax, Auto, rcx, DWord, rdx, DWord]                                             ,  [0x0F, 0x01, 0xFA  ], X, DEFAULT, AMD;
]
"montmul" = [
    &[]                                                                              ,  [0x0F, 0xA6, 0xC0  ], X, PREF_F3, CYRIX;
]
"movapd" = [
    &[Memory, OWord, AVXRegister, OWord]                                             ,  [0x0F, 0x29        ], X, ENC_MR | PREF_66, SSE2;
    &[AVXRegister, OWord, Memory, OWord]                                             ,  [0x0F, 0x28        ], X, PREF_66, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x28        ], X, PREF_66, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x29        ], X, ENC_MR | PREF_66, SSE2;
]
"movaps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x28        ], X, DEFAULT, SSE;
    &[Word, OWord, AVXRegister, OWord]                                               ,  [0x0F, 0x29        ], X, ENC_MR, SSE;
]
"movbe" = [
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x0F, 0x38, 0xF1  ], X, AUTO_SIZE | ENC_MR;
    &[Legacy, Auto, Memory, Auto]                                                    ,  [0x0F, 0x38, 0xF0  ], X, AUTO_SIZE;
]
"movd" = [
    &[Memory, DWord, AVXRegister, OWord]                                             ,  [0x0F, 0x7E        ], X, ENC_MR | PREF_66, SSE2;
    &[MMXRegister, QWord, LegacyMemory, DWord]                                       ,  [0x0F, 0x6E        ], X, DEFAULT, MMX;
    &[MMXRegister, QWord, LegacyMemory, QWord]                                       ,  [0x0F, 0x6E        ], X, WITH_REXW, MMX;
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x6E        ], X, PREF_66, SSE2;
    &[AVXRegister, OWord, LegacyMemory, DWord]                                       ,  [0x0F, 0x6E        ], X, PREF_66, SSE2;
    &[LegacyMemory, DWord, MMXRegister, QWord]                                       ,  [0x0F, 0x7E        ], X, ENC_MR, MMX;
    &[LegacyMemory, DWord, AVXRegister, OWord]                                       ,  [0x0F, 0x7E        ], X, ENC_MR | PREF_66, SSE2;
    &[LegacyMemory, QWord, MMXRegister, QWord]                                       ,  [0x0F, 0x7E        ], X, WITH_REXW | ENC_MR, MMX;
]
"movddup" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x12        ], X, PREF_F2, SSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x12        ], X, PREF_F2, SSE3;
]
"movdq2q" = [
    &[MMXRegister, QWord, AVXRegister, OWord]                                        ,  [0x0F, 0xD6        ], X, PREF_F2, SSE2;
]
"movdqa" = [
    &[Memory, OWord, AVXRegister, OWord]                                             ,  [0x0F, 0x7F        ], X, ENC_MR | PREF_66, SSE2;
    &[AVXRegister, OWord, Memory, OWord]                                             ,  [0x0F, 0x6F        ], X, PREF_66, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x6F        ], X, PREF_66, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x7F        ], X, ENC_MR | PREF_66, SSE2;
]
"movdqu" = [
    &[Memory, OWord, AVXRegister, OWord]                                             ,  [0x0F, 0x7F        ], X, ENC_MR | PREF_F3, SSE2;
    &[AVXRegister, OWord, Memory, OWord]                                             ,  [0x0F, 0x6F        ], X, PREF_F3, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x6F        ], X, PREF_F3, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x7F        ], X, ENC_MR | PREF_F3, SSE2;
]
"movhlps" = [
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x12        ], X, DEFAULT, SSE;
]
"movhpd" = [
    &[Memory, MemAuto, AVXRegister, OWord]                                           ,  [0x0F, 0x17        ], X, ENC_MR | PREF_66, SSE2;
    &[AVXRegister, OWord, Memory, MemAuto]                                           ,  [0x0F, 0x16        ], X, PREF_66, SSE2;
]
"movhps" = [
    &[Memory, QWord, AVXRegister, OWord]                                             ,  [0x0F, 0x17        ], X, ENC_MR, SSE;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x16        ], X, DEFAULT, SSE;
]
"movlhps" = [
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x16        ], X, DEFAULT, SSE;
]
"movlpd" = [
    &[Memory, QWord, AVXRegister, OWord]                                             ,  [0x0F, 0x13        ], X, ENC_MR | PREF_66, SSE2;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x12        ], X, PREF_66, SSE2;
]
"movlps" = [
    &[Memory, QWord, AVXRegister, OWord]                                             ,  [0x0F, 0x13        ], X, ENC_MR, SSE;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x12        ], X, DEFAULT, SSE;
]
"movmskpd" = [
    &[Legacy, DWord, AVXRegister, OWord]                                             ,  [0x0F, 0x50        ], X, PREF_66, SSE2;
    &[Legacy, QWord, AVXRegister, OWord]                                             ,  [0x0F, 0x50        ], X, WITH_REXW | PREF_66, SSE2;
]
"movmskps" = [
    &[Legacy, DWord, AVXRegister, OWord]                                             ,  [0x0F, 0x50        ], X, DEFAULT, SSE;
    &[Legacy, QWord, AVXRegister, OWord]                                             ,  [0x0F, 0x50        ], X, WITH_REXW, SSE;
]
"movntdq" = [
    &[Memory, OWord, AVXRegister, OWord]                                             ,  [0x0F, 0xE7        ], X, ENC_MR | PREF_66, SSE2;
]
"movntdqa" = [
    &[AVXRegister, OWord, Memory, OWord]                                             ,  [0x0F, 0x38, 0x2A  ], X, PREF_66, SSE41;
]
"movnti" = [
    &[Memory, DWord, Legacy, DWord]                                                  ,  [0x0F, 0xC3        ], X, ENC_MR;
    &[Memory, QWord, Legacy, QWord]                                                  ,  [0x0F, 0xC3        ], X, WITH_REXW | ENC_MR;
]
"movntpd" = [
    &[Memory, OWord, AVXRegister, OWord]                                             ,  [0x0F, 0x2B        ], X, ENC_MR | PREF_66, SSE2;
]
"movntps" = [
    &[Memory, OWord, AVXRegister, OWord]                                             ,  [0x0F, 0x2B        ], X, ENC_MR, SSE;
]
"movntq" = [
    &[Memory, QWord, MMXRegister, QWord]                                             ,  [0x0F, 0xE7        ], X, ENC_MR, MMX;
]
"movntsd" = [
    &[Memory, QWord, AVXRegister, OWord]                                             ,  [0x0F, 0x2B        ], X, ENC_MR | PREF_F2, AMD | SSE4A;
]
"movntss" = [
    &[Memory, DWord, AVXRegister, OWord]                                             ,  [0x0F, 0x2B        ], X, ENC_MR | PREF_F3, SSE4A | AMD;
]
"movq" = [
    &[Memory, QWord, AVXRegister, OWord]                                             ,  [0x0F, 0xD6        ], X, ENC_MR | PREF_66, SSE2;
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x6F        ], X, DEFAULT, MMX;
    &[MMXRegister, QWord, LegacyMemory, QWord]                                       ,  [0x0F, 0x6E        ], X, WITH_REXW, MMX;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x7E        ], X, PREF_F3, SSE2;
    &[AVXRegister, OWord, LegacyMemory, QWord]                                       ,  [0x0F, 0x6E        ], X, WITH_REXW | PREF_66, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x7E        ], X, PREF_F3, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0xD6        ], X, ENC_MR | PREF_66, SSE2;
    &[MMXMemory, QWord, MMXRegister, QWord]                                          ,  [0x0F, 0x7F        ], X, ENC_MR, MMX;
    &[LegacyMemory, QWord, MMXRegister, QWord]                                       ,  [0x0F, 0x7E        ], X, WITH_REXW | ENC_MR, MMX;
    &[LegacyMemory, QWord, AVXRegister, OWord]                                       ,  [0x0F, 0x7E        ], X, WITH_REXW | ENC_MR | PREF_66, SSE2;
]
"movq2dq" = [
    &[AVXRegister, OWord, MMXRegister, QWord]                                        ,  [0x0F, 0xD6        ], X, PREF_F3, SSE2;
]
"movsb" = [
    &[]                                                                              ,  [0xA4              ], X, REP;
]
"movsd" = [
    &[]                                                                              ,  [0xA5              ], X, REP;
    &[Memory, QWord, AVXRegister, OWord]                                             ,  [0x0F, 0x11        ], X, ENC_MR | PREF_F2, SSE2;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x10        ], X, PREF_F2, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x10        ], X, PREF_F2, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x11        ], X, ENC_MR | PREF_F2, SSE2;
]
"movshdup" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x16        ], X, PREF_F3, SSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x16        ], X, PREF_F3, SSE3;
]
"movsldup" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x12        ], X, PREF_F3, SSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x12        ], X, PREF_F3, SSE3;
]
"movsq" = [
    &[]                                                                              ,  [0xA5              ], X, WITH_REXW | REP;
]
"movss" = [
    &[Memory, DWord, AVXRegister, OWord]                                             ,  [0x0F, 0x11        ], X, ENC_MR | PREF_F3, SSE;
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x10        ], X, PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x10        ], X, PREF_F3, SSE;
]
"movsw" = [
    &[]                                                                              ,  [0xA5              ], X, WORD_SIZE | REP;
]
"movsx" = [
    &[Legacy, QWord, LegacyMemory, DWord]                                            ,  [0x63              ], X, WITH_REXW;
    &[Legacy, Word, Memory, Byte]                                                    ,  [0x0F, 0xBE        ], X, WORD_SIZE;
    &[Legacy, Auto, LegacyMemory, Byte]                                              ,  [0x0F, 0xBE        ], X, AUTO_SIZE;
    &[Legacy, Auto, LegacyMemory, Word]                                              ,  [0x0F, 0xBF        ], X, AUTO_REXW | EXACT_SIZE;
]
"movsxd" = [
    &[Legacy, QWord, LegacyMemory, DWord]                                            ,  [0x63              ], X, WITH_REXW;
]
"movupd" = [
    &[Memory, OWord, AVXRegister, OWord]                                             ,  [0x0F, 0x11        ], X, ENC_MR | PREF_66, SSE2;
    &[AVXRegister, OWord, Memory, OWord]                                             ,  [0x0F, 0x10        ], X, PREF_66, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x10        ], X, PREF_66, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x11        ], X, ENC_MR | PREF_66, SSE2;
]
"movups" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x10        ], X, DEFAULT, SSE;
    &[Word, OWord, AVXRegister, OWord]                                               ,  [0x0F, 0x11        ], X, ENC_MR, SSE;
]
"movzx" = [
    &[Legacy, Word, Memory, Byte]                                                    ,  [0x0F, 0xB6        ], X, WORD_SIZE;
    &[Legacy, Auto, LegacyMemory, Byte]                                              ,  [0x0F, 0xB6        ], X, AUTO_SIZE;
    &[Legacy, Auto, LegacyMemory, Word]                                              ,  [0x0F, 0xB7        ], X, AUTO_REXW | EXACT_SIZE;
]
"mpsadbw" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x42  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x42  ], X, PREF_66, SSE41;
]
"mul" = [
    &[LegacyMemory, Byte]                                                            ,  [0xF6              ], 4;
    &[LegacyMemory, Auto]                                                            ,  [0xF7              ], 4, AUTO_SIZE;
]
"mulpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x59        ], X, PREF_66, SSE2;
]
"mulps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x59        ], X, DEFAULT, SSE;
]
"mulsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x59        ], X, PREF_F2, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x59        ], X, PREF_F2, SSE2;
]
"mulss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x59        ], X, PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x59        ], X, PREF_F3, SSE;
]
"mulx" = [
    &[Legacy, Auto, Legacy, Auto, LegacyMemory, Auto]                                ,  [0x02, 0xF6        ], X, VEX_OP | AUTO_REXW | PREF_F2, BMI2;
]
"mwait" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xC9  ], X;
    &[rax, DWord, rcx, DWord]                                                        ,  [0x0F, 0x01, 0xC9  ], X;
]
"mwaitx" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xFB  ], X, DEFAULT, AMD;
    &[rax, DWord, rcx, DWord]                                                        ,  [0x0F, 0x01, 0xFB  ], X, DEFAULT, AMD;
]
"neg" = [
    &[Memory, Byte]                                                                  ,  [0xF6              ], 3, LOCK;
    &[Legacy, Byte]                                                                  ,  [0xF6              ], 3;
    &[Memory, Auto]                                                                  ,  [0xF7              ], 3, AUTO_SIZE | LOCK;
    &[Legacy, Auto]                                                                  ,  [0xF7              ], 3, AUTO_SIZE ;
]
"nop" = [
    &[]                                                                              ,  [0x90              ], X;
    &[LegacyMemory, Auto]                                                            ,  [0x0F, 0x1F        ], 0, AUTO_SIZE;
]
"not" = [
    &[Memory, Byte]                                                                  ,  [0xF6              ], 2, LOCK;
    &[Legacy, Byte]                                                                  ,  [0xF6              ], 2;
    &[Memory, Auto]                                                                  ,  [0xF7              ], 2, AUTO_SIZE | LOCK;
    &[Legacy, Auto]                                                                  ,  [0xF7              ], 2, AUTO_SIZE ;
]
"or" = [
    &[rax, Byte, Immediate, Byte]                                                    ,  [0x0C              ], X;
    &[Memory, Byte, Immediate, Byte]                                                 ,  [0x80              ], 1, LOCK;
    &[Memory, Byte, Legacy, Byte]                                                    ,  [0x08              ], X, LOCK | ENC_MR;
    &[Legacy, Byte, Immediate, Byte]                                                 ,  [0x80              ], 1;
    &[Legacy, Byte, Legacy, Byte]                                                    ,  [0x08              ], X, ENC_MR;
    &[Legacy, Byte, LegacyMemory, Byte]                                              ,  [0x0A              ], X;
    &[Legacy, Auto, Immediate, Byte]                                                 ,  [0x83              ], 1, AUTO_SIZE  | EXACT_SIZE;
    &[rax, Auto, Immediate, Auto]                                                    ,  [0x0D              ], X, AUTO_SIZE;
    &[Memory, Auto, Immediate, Auto]                                                 ,  [0x81              ], 1, AUTO_SIZE | LOCK;
    &[Memory, Auto, Immediate, Byte]                                                 ,  [0x83              ], 1, AUTO_SIZE | LOCK;
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x09              ], X, AUTO_SIZE | LOCK | ENC_MR;
    &[Legacy, Auto, Immediate, Auto]                                                 ,  [0x81              ], 1, AUTO_SIZE ;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x09              ], X, AUTO_SIZE | ENC_MR;
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x0B              ], X, AUTO_SIZE;
]
"orpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x56        ], X, PREF_66, SSE2;
]
"orps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x56        ], X, DEFAULT, SSE;
]
"outsb" = [
    &[]                                                                              ,  [0x6E              ], X, REP;
]
"outsd" = [
    &[]                                                                              ,  [0x6F              ], X, REP;
]
"outsw" = [
    &[]                                                                              ,  [0x6F              ], X, WORD_SIZE | REP;
]
"pabsb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x1C  ], X, DEFAULT, MMX | SSSE3;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x1C  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x1C  ], X, PREF_66, SSSE3;
]
"pabsd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x1E  ], X, DEFAULT, MMX | SSSE3;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x1E  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x1E  ], X, PREF_66, SSSE3;
]
"pabsw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x1D  ], X, DEFAULT, SSSE3 | MMX;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x1D  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x1D  ], X, PREF_66, SSSE3;
]
"packssdw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x6B        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x6B        ], X, PREF_66, SSE2;
]
"packsswb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x63        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x63        ], X, PREF_66, SSE2;
]
"packusdw" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x2B  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x2B  ], X, PREF_66, SSE41;
]
"packuswb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x67        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x67        ], X, PREF_66, SSE2;
]
"paddb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xFC        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xFC        ], X, PREF_66, SSE2;
]
"paddd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xFE        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xFE        ], X, PREF_66, SSE2;
]
"paddq" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xD4        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xD4        ], X, PREF_66, SSE2;
]
"paddsb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xEC        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xEC        ], X, PREF_66, SSE2;
]
"paddsiw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x51        ], X, DEFAULT, MMX | CYRIX;
]
"paddsw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xED        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xED        ], X, PREF_66, SSE2;
]
"paddusb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xDC        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xDC        ], X, PREF_66, SSE2;
]
"paddusw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xDD        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xDD        ], X, PREF_66, SSE2;
]
"paddw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xFD        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xFD        ], X, PREF_66, SSE2;
]
"palignr" = [
    &[MMXRegister, QWord, MMXMemory, QWord, Immediate, Byte]                         ,  [0x0F, 0x3A, 0x0F  ], X, DEFAULT, SSSE3 | MMX;
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x0F  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x0F  ], X, PREF_66, SSSE3;
]
"pand" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xDB        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xDB        ], X, PREF_66, SSE2;
]
"pandn" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xDF        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xDF        ], X, PREF_66, SSE2;
]
"pause" = [
    &[]                                                                              ,  [0x90              ], X, PREF_F3;
]
"paveb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x50        ], X, DEFAULT, MMX | CYRIX;
]
"pavgb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xE0        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xE0        ], X, PREF_66, SSE2;
]
"pavgusb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0xBF  ], X, IMM_OP, TDNOW;
]
"pavgw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xE3        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xE3        ], X, PREF_66, SSE2;
]
"pblendvb" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x10  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x10  ], X, PREF_66, SSE41;
]
"pblendw" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x0E  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x0E  ], X, PREF_66, SSE41;
]
"pclmulhqhqdq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x3A, 0x44, 0x11], X, IMM_OP | PREF_66, SSE;
]
"pclmulhqlqdq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x3A, 0x44, 0x01], X, PREF_66 | IMM_OP, SSE;
]
"pclmullqhqdq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x3A, 0x44, 0x10], X, PREF_66 | IMM_OP, SSE;
]
"pclmullqlqdq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x3A, 0x44, 0x00], X, PREF_66 | IMM_OP, SSE;
]
"pclmulqdq" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x0F, 0x3A, 0x44  ], X, PREF_66, SSE;
]
"pcmpeqb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x74        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x74        ], X, PREF_66, SSE2;
]
"pcmpeqd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x76        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x76        ], X, PREF_66, SSE2;
]
"pcmpeqq" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x29  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x29  ], X, PREF_66, SSE41;
]
"pcmpeqw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x75        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x75        ], X, PREF_66, SSE2;
]
"pcmpestri" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x61  ], X, PREF_66, SSE42;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x61  ], X, PREF_66, SSE42;
]
"pcmpestrm" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x60  ], X, PREF_66, SSE42;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x60  ], X, PREF_66, SSE42;
]
"pcmpgtb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x64        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x64        ], X, PREF_66, SSE2;
]
"pcmpgtd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x66        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x66        ], X, PREF_66, SSE2;
]
"pcmpgtq" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x37  ], X, PREF_66, SSE42;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x37  ], X, PREF_66, SSE42;
]
"pcmpgtw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x65        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x65        ], X, PREF_66, SSE2;
]
"pcmpistri" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x63  ], X, PREF_66, SSE42;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x63  ], X, PREF_66, SSE42;
]
"pcmpistrm" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x62  ], X, PREF_66, SSE42;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x62  ], X, PREF_66, SSE42;
]
"pdep" = [
    &[Legacy, Auto, Legacy, Auto, LegacyMemory, Auto]                                ,  [0x02, 0xF5        ], X, VEX_OP | AUTO_REXW | PREF_F2, BMI2;
]
"pdistib" = [
    &[MMXRegister, QWord, Memory, QWord]                                             ,  [0x0F, 0x54        ], X, DEFAULT, MMX | CYRIX;
]
"pext" = [
    &[Legacy, Auto, Legacy, Auto, LegacyMemory, Auto]                                ,  [0x02, 0xF5        ], X, VEX_OP | AUTO_REXW | PREF_F3, BMI2;
]
"pextrb" = [
    &[Memory, Byte, AVXRegister, OWord, Immediate, Byte]                             ,  [0x0F, 0x3A, 0x14  ], X, ENC_MR | PREF_66, SSE41;
    &[Legacy, DWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x14  ], X, ENC_MR | PREF_66, SSE41;
    &[Legacy, QWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x14  ], X, WITH_REXW | ENC_MR | PREF_66, SSE41;
]
"pextrd" = [
    &[LegacyMemory, DWord, AVXRegister, OWord, Immediate, Byte]                      ,  [0x0F, 0x3A, 0x16  ], X, ENC_MR | PREF_66, SSE41;
]
"pextrq" = [
    &[LegacyMemory, QWord, AVXRegister, OWord, Immediate, Byte]                      ,  [0x0F, 0x3A, 0x16  ], X, WITH_REXW | ENC_MR | PREF_66, SSE41;
]
"pextrw" = [
    &[Memory, Word, AVXRegister, OWord, Immediate, Byte]                             ,  [0x0F, 0x3A, 0x15  ], X, ENC_MR | PREF_66, SSE41;
    &[Legacy, DWord, MMXRegister, QWord, Immediate, Byte]                            ,  [0x0F, 0xC5        ], X, DEFAULT, MMX;
    &[Legacy, DWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x0F, 0xC5        ], X, PREF_66, SSE2;
    &[Legacy, DWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x15  ], X, ENC_MR | PREF_66, SSE41;
    &[Legacy, QWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x15  ], X, WITH_REXW | ENC_MR | PREF_66, SSE41;
]
"pf2id" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x1D  ], X, IMM_OP, TDNOW;
]
"pf2iw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x1C  ], X, IMM_OP, TDNOW;
]
"pfacc" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0xAE  ], X, IMM_OP, TDNOW;
]
"pfadd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x9E  ], X, IMM_OP, TDNOW;
]
"pfcmpeq" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0xB0  ], X, IMM_OP, TDNOW;
]
"pfcmpge" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x90  ], X, IMM_OP, TDNOW;
]
"pfcmpgt" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0xA0  ], X, IMM_OP, TDNOW;
]
"pfmax" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0xA4  ], X, IMM_OP, TDNOW;
]
"pfmin" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x94  ], X, IMM_OP, TDNOW;
]
"pfmul" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0xB4  ], X, IMM_OP, TDNOW;
]
"pfnacc" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x8A  ], X, IMM_OP, TDNOW;
]
"pfpnacc" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x8E  ], X, IMM_OP, TDNOW;
]
"pfrcp" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x96  ], X, IMM_OP, TDNOW;
]
"pfrcpit1" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0xA6  ], X, IMM_OP, TDNOW;
]
"pfrcpit2" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0xB6  ], X, IMM_OP, TDNOW;
]
"pfrcpv" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x86  ], X, IMM_OP, TDNOW | CYRIX;
]
"pfrsqit1" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0xA7  ], X, IMM_OP, TDNOW;
]
"pfrsqrt" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x97  ], X, IMM_OP, TDNOW;
]
"pfrsqrtv" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x87  ], X, IMM_OP, CYRIX | TDNOW;
]
"pfsub" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x9A  ], X, IMM_OP, TDNOW;
]
"pfsubr" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0xAA  ], X, IMM_OP, TDNOW;
]
"phaddd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x02  ], X, DEFAULT, MMX | SSSE3;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x02  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x02  ], X, PREF_66, SSSE3;
]
"phaddsw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x03  ], X, DEFAULT, MMX | SSSE3;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x03  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x03  ], X, PREF_66, SSSE3;
]
"phaddw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x01  ], X, DEFAULT, MMX | SSSE3;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x01  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x01  ], X, PREF_66, SSSE3;
]
"phminposuw" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x41  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x41  ], X, PREF_66, SSE41;
]
"phsubd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x06  ], X, DEFAULT, SSSE3 | MMX;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x06  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x06  ], X, PREF_66, SSSE3;
]
"phsubsw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x07  ], X, DEFAULT, SSSE3 | MMX;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x07  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x07  ], X, PREF_66, SSSE3;
]
"phsubw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x05  ], X, DEFAULT, SSSE3 | MMX;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x05  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x05  ], X, PREF_66, SSSE3;
]
"pi2fd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x0D  ], X, IMM_OP, TDNOW;
]
"pi2fw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0x0C  ], X, IMM_OP, TDNOW;
]
"pinsrb" = [
    &[AVXRegister, OWord, Memory, MemAuto, Immediate, Byte]                          ,  [0x0F, 0x3A, 0x20  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, Legacy, DWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x20  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, LegacyMemory, Byte, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x20  ], X, PREF_66, SSE41;
]
"pinsrd" = [
    &[AVXRegister, OWord, Memory, MemAuto, Immediate, Byte]                          ,  [0x0F, 0x3A, 0x22  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, LegacyMemory, DWord, Immediate, Byte]                      ,  [0x0F, 0x3A, 0x22  ], X, PREF_66, SSE41;
]
"pinsrq" = [
    &[AVXRegister, OWord, Memory, MemAuto, Immediate, Byte]                          ,  [0x0F, 0x3A, 0x22  ], X, WITH_REXW | PREF_66, SSE41;
    &[AVXRegister, OWord, LegacyMemory, QWord, Immediate, Byte]                      ,  [0x0F, 0x3A, 0x22  ], X, WITH_REXW | PREF_66, SSE41;
]
"pinsrw" = [
    &[MMXRegister, QWord, Memory, MemAuto, Immediate, Byte]                          ,  [0x0F, 0xC4        ], X, DEFAULT, MMX;
    &[MMXRegister, QWord, Legacy, DWord, Immediate, Byte]                            ,  [0x0F, 0xC4        ], X, DEFAULT, MMX;
    &[MMXRegister, QWord, LegacyMemory, Word, Immediate, Byte]                       ,  [0x0F, 0xC4        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Memory, MemAuto, Immediate, Byte]                          ,  [0x0F, 0xC4        ], X, PREF_66, SSE2;
    &[AVXRegister, OWord, Memory, Word, Immediate, Byte]                             ,  [0x0F, 0xC4        ], X, PREF_66, SSE2;
    &[AVXRegister, OWord, Legacy, DWord, Immediate, Byte]                            ,  [0x0F, 0xC4        ], X, PREF_66, SSE2;
    &[AVXRegister, OWord, Legacy, Word, Immediate, Byte]                             ,  [0x0F, 0xC4        ], X, PREF_66, SSE2;
]
"pmachriw" = [
    &[MMXRegister, QWord, Memory, QWord]                                             ,  [0x0F, 0x5E        ], X, DEFAULT, MMX | CYRIX;
]
"pmaddubsw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x04  ], X, DEFAULT, MMX | SSSE3;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x04  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x04  ], X, PREF_66, SSSE3;
]
"pmaddwd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xF5        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xF5        ], X, PREF_66, SSE2;
]
"pmagw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x52        ], X, DEFAULT, CYRIX | MMX;
]
"pmaxsb" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x3C  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x3C  ], X, PREF_66, SSE41;
]
"pmaxsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x3D  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x3D  ], X, PREF_66, SSE41;
]
"pmaxsw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xEE        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xEE        ], X, PREF_66, SSE2;
]
"pmaxub" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xDE        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xDE        ], X, PREF_66, SSE2;
]
"pmaxud" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x3F  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x3F  ], X, PREF_66, SSE41;
]
"pmaxuw" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x3E  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x3E  ], X, PREF_66, SSE41;
]
"pminsb" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x38  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x38  ], X, PREF_66, SSE41;
]
"pminsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x39  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x39  ], X, PREF_66, SSE41;
]
"pminsw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xEA        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xEA        ], X, PREF_66, SSE2;
]
"pminub" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xDA        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xDA        ], X, PREF_66, SSE2;
]
"pminud" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x3B  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x3B  ], X, PREF_66, SSE41;
]
"pminuw" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x3A  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x3A  ], X, PREF_66, SSE41;
]
"pmovmskb" = [
    &[Legacy, DWord, MMXRegister, QWord]                                             ,  [0x0F, 0xD7        ], X, DEFAULT, MMX;
    &[Legacy, DWord, AVXRegister, OWord]                                             ,  [0x0F, 0xD7        ], X, PREF_66, SSE2;
]
"pmovsxbd" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x38, 0x21  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x21  ], X, PREF_66, SSE41;
]
"pmovsxbq" = [
    &[AVXRegister, OWord, Memory, Word]                                              ,  [0x0F, 0x38, 0x22  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x22  ], X, PREF_66, SSE41;
]
"pmovsxbw" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x20  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x20  ], X, PREF_66, SSE41;
]
"pmovsxdq" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x25  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x25  ], X, PREF_66, SSE41;
]
"pmovsxwd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x23  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x23  ], X, PREF_66, SSE41;
]
"pmovsxwq" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x38, 0x24  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x24  ], X, PREF_66, SSE41;
]
"pmovzxbd" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x38, 0x31  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x31  ], X, PREF_66, SSE41;
]
"pmovzxbq" = [
    &[AVXRegister, OWord, Memory, Word]                                              ,  [0x0F, 0x38, 0x32  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x32  ], X, PREF_66, SSE41;
]
"pmovzxbw" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x30  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x30  ], X, PREF_66, SSE41;
]
"pmovzxdq" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x35  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x35  ], X, PREF_66, SSE41;
]
"pmovzxwd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x33  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x33  ], X, PREF_66, SSE41;
]
"pmovzxwq" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x38, 0x34  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x34  ], X, PREF_66, SSE41;
]
"pmuldq" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x28  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x28  ], X, PREF_66, SSE41;
]
"pmulhriw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x5D        ], X, DEFAULT, CYRIX | MMX;
]
"pmulhrsw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x0B  ], X, DEFAULT, MMX | SSSE3;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x0B  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x0B  ], X, PREF_66, SSSE3;
]
"pmulhrwa" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0xB7  ], X, IMM_OP, TDNOW;
]
"pmulhrwc" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x59        ], X, DEFAULT, MMX | CYRIX;
]
"pmulhuw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xE4        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xE4        ], X, PREF_66, SSE2;
]
"pmulhw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xE5        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xE5        ], X, PREF_66, SSE2;
]
"pmulld" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x40  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x40  ], X, PREF_66, SSE41;
]
"pmullw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xD5        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xD5        ], X, PREF_66, SSE2;
]
"pmuludq" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xF4        ], X, DEFAULT, SSE2;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xF4        ], X, PREF_66, SSE2;
]
"pmvgezb" = [
    &[MMXRegister, QWord, Memory, QWord]                                             ,  [0x0F, 0x5C        ], X, DEFAULT, CYRIX | MMX;
]
"pmvlzb" = [
    &[MMXRegister, QWord, Memory, QWord]                                             ,  [0x0F, 0x5B        ], X, DEFAULT, CYRIX | MMX;
]
"pmvnzb" = [
    &[MMXRegister, QWord, Memory, QWord]                                             ,  [0x0F, 0x5A        ], X, DEFAULT, CYRIX | MMX;
]
"pmvzb" = [
    &[MMXRegister, QWord, Memory, QWord]                                             ,  [0x0F, 0x58        ], X, DEFAULT, MMX | CYRIX;
]
"pop" = [
    &[es, Word]                                                                      ,  [0x07              ], X, X86_ONLY;
    &[ss, Word]                                                                      ,  [0x17              ], X, X86_ONLY;
    &[ds, Word]                                                                      ,  [0x1F              ], X, X86_ONLY;
    &[fs, Word]                                                                      ,  [0x0F, 0xA1        ], X;
    &[gs, Word]                                                                      ,  [0x0F, 0xA9        ], X;
    &[Legacy, Auto]                                                                  ,  [0x58              ], X, AUTO_NO32 | SHORT_ARG;
    &[LegacyMemory, Auto]                                                            ,  [0x8F              ], 0, AUTO_NO32;
]
"popa" = [
    &[]                                                                              ,  [0x61              ], X, X86_ONLY | WORD_SIZE;
]
"popad" = [
    &[]                                                                              ,  [0x61              ], X, X86_ONLY;
]
"popcnt" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x0F, 0xB8        ], X, AUTO_SIZE | PREF_F3;
]
"popf" = [
    &[]                                                                              ,  [0x9D              ], X;
]
"popfq" = [
    &[]                                                                              ,  [0x9D              ], X;
]
"popfw" = [
    &[]                                                                              ,  [0x9D              ], X, WORD_SIZE;
]
"por" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xEB        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xEB        ], X, PREF_66, SSE2;
]
"prefetch" = [
    &[Memory, QWord]                                                                 ,  [0x0F, 0x0D        ], 0, DEFAULT, TDNOW;
]
"prefetchnta" = [
    &[Memory, Byte]                                                                  ,  [0x0F, 0x18        ], 0;
]
"prefetcht0" = [
    &[Memory, Byte]                                                                  ,  [0x0F, 0x18        ], 1;
]
"prefetcht1" = [
    &[Memory, Byte]                                                                  ,  [0x0F, 0x18        ], 2;
]
"prefetcht2" = [
    &[Memory, Byte]                                                                  ,  [0x0F, 0x18        ], 3;
]
"prefetchw" = [
    &[Memory, QWord]                                                                 ,  [0x0F, 0x0D        ], 1, DEFAULT, TDNOW;
]
"prefetchwt1" = [
    &[Memory, Byte]                                                                  ,  [0x0F, 0x0D        ], 2, DEFAULT, PREFETCHWT1;
]
"psadbw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xF6        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xF6        ], X, PREF_66, SSE2;
]
"pshufb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x00  ], X, DEFAULT, SSSE3 | MMX;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x00  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x00  ], X, PREF_66, SSSE3;
]
"pshufd" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x0F, 0x70        ], X, PREF_66, SSE2;
]
"pshufhw" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x0F, 0x70        ], X, PREF_F3, SSE2;
]
"pshuflw" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x0F, 0x70        ], X, PREF_F2, SSE2;
]
"pshufw" = [
    &[MMXRegister, QWord, MMXMemory, QWord, Immediate, Byte]                         ,  [0x0F, 0x70        ], X, DEFAULT, MMX;
]
"psignb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x08  ], X, DEFAULT, MMX | SSSE3;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x08  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x08  ], X, PREF_66, SSSE3;
]
"psignd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x0A  ], X, DEFAULT, SSSE3 | MMX;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x0A  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x0A  ], X, PREF_66, SSSE3;
]
"psignw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x38, 0x09  ], X, DEFAULT, MMX | SSSE3;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x09  ], X, PREF_66, SSSE3;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x09  ], X, PREF_66, SSSE3;
]
"pslld" = [
    &[MMXRegister, QWord, Immediate, Byte]                                           ,  [0x0F, 0x72        ], 6, DEFAULT, MMX;
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xF2        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Immediate, Byte]                                           ,  [0x0F, 0x72        ], 6, PREF_66, SSE2;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xF2        ], X, PREF_66, SSE2;
]
"pslldq" = [
    &[AVXRegister, OWord, Immediate, Byte]                                           ,  [0x0F, 0x73        ], 7, PREF_66, SSE2;
]
"psllq" = [
    &[MMXRegister, QWord, Immediate, Byte]                                           ,  [0x0F, 0x73        ], 6, DEFAULT, MMX;
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xF3        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Immediate, Byte]                                           ,  [0x0F, 0x73        ], 6, PREF_66, SSE2;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xF3        ], X, PREF_66, SSE2;
]
"psllw" = [
    &[MMXRegister, QWord, Immediate, Byte]                                           ,  [0x0F, 0x71        ], 6, DEFAULT, MMX;
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xF1        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Immediate, Byte]                                           ,  [0x0F, 0x71        ], 6, PREF_66, SSE2;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xF1        ], X, PREF_66, SSE2;
]
"psrad" = [
    &[MMXRegister, QWord, Immediate, Byte]                                           ,  [0x0F, 0x72        ], 4, DEFAULT, MMX;
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xE2        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Immediate, Byte]                                           ,  [0x0F, 0x72        ], 4, PREF_66, SSE2;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xE2        ], X, PREF_66, SSE2;
]
"psraw" = [
    &[MMXRegister, QWord, Immediate, Byte]                                           ,  [0x0F, 0x71        ], 4, DEFAULT, MMX;
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xE1        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Immediate, Byte]                                           ,  [0x0F, 0x71        ], 4, PREF_66, SSE2;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xE1        ], X, PREF_66, SSE2;
]
"psrld" = [
    &[MMXRegister, QWord, Immediate, Byte]                                           ,  [0x0F, 0x72        ], 2, DEFAULT, MMX;
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xD2        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Immediate, Byte]                                           ,  [0x0F, 0x72        ], 2, PREF_66, SSE2;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xD2        ], X, PREF_66, SSE2;
]
"psrldq" = [
    &[AVXRegister, OWord, Immediate, Byte]                                           ,  [0x0F, 0x73        ], 3, PREF_66, SSE2;
]
"psrlq" = [
    &[MMXRegister, QWord, Immediate, Byte]                                           ,  [0x0F, 0x73        ], 2, DEFAULT, MMX;
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xD3        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Immediate, Byte]                                           ,  [0x0F, 0x73        ], 2, PREF_66, SSE2;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xD3        ], X, PREF_66, SSE2;
]
"psrlw" = [
    &[MMXRegister, QWord, Immediate, Byte]                                           ,  [0x0F, 0x71        ], 2, DEFAULT, MMX;
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xD1        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Immediate, Byte]                                           ,  [0x0F, 0x71        ], 2, PREF_66, SSE2;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xD1        ], X, PREF_66, SSE2;
]
"psubb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xF8        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xF8        ], X, PREF_66, SSE2;
]
"psubd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xFA        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xFA        ], X, PREF_66, SSE2;
]
"psubq" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xFB        ], X, DEFAULT, SSE2;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xFB        ], X, PREF_66, SSE2;
]
"psubsb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xE8        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xE8        ], X, PREF_66, SSE2;
]
"psubsiw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x55        ], X, DEFAULT, CYRIX | MMX;
]
"psubsw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xE9        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xE9        ], X, PREF_66, SSE2;
]
"psubusb" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xD8        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xD8        ], X, PREF_66, SSE2;
]
"psubusw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xD9        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xD9        ], X, PREF_66, SSE2;
]
"psubw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xF9        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xF9        ], X, PREF_66, SSE2;
]
"pswapd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x0F, 0xBB  ], X, IMM_OP, TDNOW;
]
"ptest" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x38, 0x17  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x38, 0x17  ], X, PREF_66, SSE41;
]
"punpckhbw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x68        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x68        ], X, PREF_66, SSE2;
]
"punpckhdq" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x6A        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x6A        ], X, PREF_66, SSE2;
]
"punpckhqdq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x6D        ], X, PREF_66, SSE2;
]
"punpckhwd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x69        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x69        ], X, PREF_66, SSE2;
]
"punpcklbw" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x60        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x60        ], X, PREF_66, SSE2;
]
"punpckldq" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x62        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x62        ], X, PREF_66, SSE2;
]
"punpcklqdq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x6C        ], X, PREF_66, SSE2;
]
"punpcklwd" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0x61        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x61        ], X, PREF_66, SSE2;
]
"push" = [
    &[es, Word]                                                                      ,  [0x06              ], X, X86_ONLY;
    &[cs, Word]                                                                      ,  [0x0E              ], X, X86_ONLY;
    &[ss, Word]                                                                      ,  [0x16              ], X, X86_ONLY;
    &[ds, Word]                                                                      ,  [0x1E              ], X, X86_ONLY;
    &[fs, Word]                                                                      ,  [0x0F, 0xA0        ], X;
    &[gs, Word]                                                                      ,  [0x0F, 0xA8        ], X;
    &[Immediate, Byte]                                                               ,  [0x6A              ], X, EXACT_SIZE;
    &[Immediate, Word]                                                               ,  [0x68              ], X, EXACT_SIZE | WORD_SIZE;
    &[Immediate, DWord]                                                              ,  [0x68              ], X;
    &[Legacy, Auto]                                                                  ,  [0x50              ], X, AUTO_NO32 | SHORT_ARG;
    &[LegacyMemory, Auto]                                                            ,  [0xFF              ], 6, AUTO_NO32;
]
"pusha" = [
    &[]                                                                              ,  [0x60              ], X, X86_ONLY | WORD_SIZE;
]
"pushad" = [
    &[]                                                                              ,  [0x60              ], X, X86_ONLY;
]
"pushf" = [
    &[]                                                                              ,  [0x9C              ], X;
]
"pushfq" = [
    &[]                                                                              ,  [0x9C              ], X;
]
"pushfw" = [
    &[]                                                                              ,  [0x9C              ], X, WORD_SIZE;
]
"pxor" = [
    &[MMXRegister, QWord, MMXMemory, QWord]                                          ,  [0x0F, 0xEF        ], X, DEFAULT, MMX;
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0xEF        ], X, PREF_66, SSE2;
]
"rcl" = [
    &[LegacyMemory, Byte, rcx, Byte]                                                 ,  [0xD2              ], 2;
    &[LegacyMemory, Byte, Immediate, Byte]                                           ,  [0xC0              ], 2;
    &[LegacyMemory, Auto, rcx, Byte]                                                 ,  [0xD3              ], 2, AUTO_SIZE;
    &[LegacyMemory, Auto, Immediate, Byte]                                           ,  [0xC1              ], 2, AUTO_SIZE;
]
"rcpps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x53        ], X, DEFAULT, SSE;
]
"rcpss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x53        ], X, PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x53        ], X, PREF_F3, SSE;
]
"rcr" = [
    &[LegacyMemory, Byte, rcx, Byte]                                                 ,  [0xD2              ], 3;
    &[LegacyMemory, Byte, Immediate, Byte]                                           ,  [0xC0              ], 3;
    &[LegacyMemory, Auto, rcx, Byte]                                                 ,  [0xD3              ], 3, AUTO_SIZE;
    &[LegacyMemory, Auto, Immediate, Byte]                                           ,  [0xC1              ], 3, AUTO_SIZE;
]
"rdfsbase" = [
    &[Legacy, DWord]                                                                 ,  [0x0F, 0xAE        ], 0, PREF_F3;
    &[Legacy, QWord]                                                                 ,  [0x0F, 0xAE        ], 0, WITH_REXW | PREF_F3;
]
"rdgsbase" = [
    &[Legacy, DWord]                                                                 ,  [0x0F, 0xAE        ], 1, PREF_F3;
    &[Legacy, QWord]                                                                 ,  [0x0F, 0xAE        ], 1, WITH_REXW | PREF_F3;
]
"rdm" = [
    &[]                                                                              ,  [0x0F, 0x3A        ], X, DEFAULT, CYRIX;
]
"rdmsr" = [
    &[]                                                                              ,  [0x0F, 0x32        ], X;
]
"rdpid" = [
    &[Legacy, QWord]                                                                 ,  [0x0F, 0xC7        ], 7, PREF_F3;
]
"rdpkru" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xEE  ], X;
]
"rdpmc" = [
    &[]                                                                              ,  [0x0F, 0x33        ], X;
]
"rdrand" = [
    &[Legacy, QWord]                                                                 ,  [0x0F, 0xC7        ], 6, WITH_REXW;
]
"rdseed" = [
    &[Legacy, QWord]                                                                 ,  [0x0F, 0xC7        ], 7, WITH_REXW;
]
"rdshr" = [
    &[LegacyMemory, DWord]                                                           ,  [0x0F, 0x36        ], 0, DEFAULT, CYRIX;
]
"rdtsc" = [
    &[]                                                                              ,  [0x0F, 0x31        ], X;
]
"rdtscp" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xF9  ], X;
]
"ret" = [
    &[]                                                                              ,  [0xC3              ], X;
    &[Immediate, Word]                                                               ,  [0xC2              ], X;
]
"retf" = [
    &[]                                                                              ,  [0xCB              ], X;
    &[Immediate, Word]                                                               ,  [0xCA              ], X;
]
"retn" = [
    &[]                                                                              ,  [0xC3              ], X;
    &[Immediate, Word]                                                               ,  [0xC2              ], X;
]
"rol" = [
    &[LegacyMemory, Byte, rcx, Byte]                                                 ,  [0xD2              ], 0;
    &[LegacyMemory, Byte, Immediate, Byte]                                           ,  [0xC0              ], 0;
    &[LegacyMemory, Auto, rcx, Byte]                                                 ,  [0xD3              ], 0, AUTO_SIZE;
    &[LegacyMemory, Auto, Immediate, Byte]                                           ,  [0xC1              ], 0, AUTO_SIZE;
]
"ror" = [
    &[LegacyMemory, Byte, rcx, Byte]                                                 ,  [0xD2              ], 1;
    &[LegacyMemory, Byte, Immediate, Byte]                                           ,  [0xC0              ], 1;
    &[LegacyMemory, Auto, rcx, Byte]                                                 ,  [0xD3              ], 1, AUTO_SIZE;
    &[LegacyMemory, Auto, Immediate, Byte]                                           ,  [0xC1              ], 1, AUTO_SIZE;
]
"rorx" = [
    &[Legacy, Auto, LegacyMemory, Auto, Immediate, Byte]                             ,  [0x03, 0xF0        ], X, VEX_OP | AUTO_REXW | PREF_F2, BMI2;
]
"roundpd" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x09  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x09  ], X, PREF_66, SSE41;
]
"roundps" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x08  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x08  ], X, PREF_66, SSE41;
]
"roundsd" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x0B  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x0B  ], X, PREF_66, SSE41;
]
"roundss" = [
    &[AVXRegister, OWord, Memory, QWord, Immediate, Byte]                            ,  [0x0F, 0x3A, 0x0A  ], X, PREF_66, SSE41;
    &[AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]                       ,  [0x0F, 0x3A, 0x0A  ], X, PREF_66, SSE41;
]
"rsdc" = [
    &[Segment, Word, Memory, PWord]                                                  ,  [0x0F, 0x79        ], X, EXACT_SIZE, CYRIX;
]
"rsldt" = [
    &[Memory, PWord]                                                                 ,  [0x0F, 0x7B        ], 0, EXACT_SIZE, CYRIX;
]
"rsm" = [
    &[]                                                                              ,  [0x0F, 0xAA        ], X;
]
"rsqrtps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x52        ], X, DEFAULT, SSE;
]
"rsqrtss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x52        ], X, PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x52        ], X, PREF_F3, SSE;
]
"rsts" = [
    &[Memory, PWord]                                                                 ,  [0x0F, 0x7D        ], 0, EXACT_SIZE, CYRIX;
]
"sahf" = [
    &[]                                                                              ,  [0x9E              ], X;
]
"sal" = [
    &[LegacyMemory, Byte, rcx, Byte]                                                 ,  [0xD2              ], 4;
    &[LegacyMemory, Byte, Immediate, Byte]                                           ,  [0xC0              ], 4;
    &[LegacyMemory, Auto, rcx, Byte]                                                 ,  [0xD3              ], 4, AUTO_SIZE;
    &[LegacyMemory, Auto, Immediate, Byte]                                           ,  [0xC1              ], 4, AUTO_SIZE;
]
"sar" = [
    &[LegacyMemory, Byte, rcx, Byte]                                                 ,  [0xD2              ], 7;
    &[LegacyMemory, Byte, Immediate, Byte]                                           ,  [0xC0              ], 7;
    &[LegacyMemory, Auto, rcx, Byte]                                                 ,  [0xD3              ], 7, AUTO_SIZE;
    &[LegacyMemory, Auto, Immediate, Byte]                                           ,  [0xC1              ], 7, AUTO_SIZE;
]
"sarx" = [
    &[Legacy, Auto, LegacyMemory, Auto, Legacy, Auto]                                ,  [0x02, 0xF7        ], X, VEX_OP | AUTO_REXW | ENC_MR | PREF_F3, BMI2;
]
"sbb" = [
    &[rax, Byte, Immediate, Byte]                                                    ,  [0x1C              ], X;
    &[Memory, Byte, Immediate, Byte]                                                 ,  [0x80              ], 3, LOCK;
    &[Memory, Byte, Legacy, Byte]                                                    ,  [0x18              ], X, LOCK | ENC_MR;
    &[Legacy, Byte, Immediate, Byte]                                                 ,  [0x80              ], 3;
    &[Legacy, Byte, Legacy, Byte]                                                    ,  [0x18              ], X, ENC_MR;
    &[Legacy, Byte, LegacyMemory, Byte]                                              ,  [0x1A              ], X;
    &[Legacy, Auto, Immediate, Byte]                                                 ,  [0x83              ], 3, AUTO_SIZE  | EXACT_SIZE;
    &[rax, Auto, Immediate, Auto]                                                    ,  [0x1D              ], X, AUTO_SIZE;
    &[Memory, Auto, Immediate, Auto]                                                 ,  [0x81              ], 3, AUTO_SIZE | LOCK;
    &[Memory, Auto, Immediate, Byte]                                                 ,  [0x83              ], 3, AUTO_SIZE | LOCK;
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x19              ], X, AUTO_SIZE | LOCK | ENC_MR;
    &[Legacy, Auto, Immediate, Auto]                                                 ,  [0x81              ], 3, AUTO_SIZE ;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x19              ], X, AUTO_SIZE | ENC_MR;
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x1B              ], X, AUTO_SIZE;
]
"scasb" = [
    &[]                                                                              ,  [0xAE              ], X, REPE;
]
"scasd" = [
    &[]                                                                              ,  [0xAF              ], X, REPE;
]
"scasq" = [
    &[]                                                                              ,  [0xAF              ], X, REPE | WITH_REXW;
]
"scasw" = [
    &[]                                                                              ,  [0xAF              ], X, REPE | WORD_SIZE;
]
"sfence" = [
    &[]                                                                              ,  [0x0F, 0xAE, 0xF8  ], X, DEFAULT, AMD;
]
"sgdt" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0x01        ], 0;
]
"sha1msg1" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x38, 0xC9  ], X, DEFAULT, SHA;
]
"sha1msg2" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x38, 0xCA  ], X, DEFAULT, SHA;
]
"sha1nexte" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x38, 0xC8  ], X, DEFAULT, SHA;
]
"sha1rnds4" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x0F, 0x3A, 0xCC  ], X, DEFAULT, SHA;
]
"sha256msg1" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x38, 0xCC  ], X, DEFAULT, SHA;
]
"sha256msg2" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x38, 0xCD  ], X, DEFAULT, SHA;
]
"sha256rnds2" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x38, 0xCB  ], X, DEFAULT, SHA;
]
"shl" = [
    &[LegacyMemory, Byte, rcx, Byte]                                                 ,  [0xD2              ], 4;
    &[LegacyMemory, Byte, Immediate, Byte]                                           ,  [0xC0              ], 4;
    &[LegacyMemory, Auto, rcx, Byte]                                                 ,  [0xD3              ], 4, AUTO_SIZE;
    &[LegacyMemory, Auto, Immediate, Byte]                                           ,  [0xC1              ], 4, AUTO_SIZE;
]
"shld" = [
    &[LegacyMemory, Auto, Legacy, Auto, rcx, Byte]                                   ,  [0x0F, 0xA5        ], X, AUTO_SIZE | ENC_MR;
    &[LegacyMemory, Auto, Legacy, Auto, Immediate, Byte]                             ,  [0x0F, 0xA4        ], X, AUTO_SIZE | ENC_MR;
]
"shlx" = [
    &[Legacy, Auto, LegacyMemory, Auto, Legacy, Auto]                                ,  [0x02, 0xF7        ], X, VEX_OP | AUTO_REXW | ENC_MR | PREF_66, BMI2;
]
"shr" = [
    &[LegacyMemory, Byte, rcx, Byte]                                                 ,  [0xD2              ], 5;
    &[LegacyMemory, Byte, Immediate, Byte]                                           ,  [0xC0              ], 5;
    &[LegacyMemory, Auto, rcx, Byte]                                                 ,  [0xD3              ], 5, AUTO_SIZE;
    &[LegacyMemory, Auto, Immediate, Byte]                                           ,  [0xC1              ], 5, AUTO_SIZE;
]
"shrd" = [
    &[LegacyMemory, Auto, Legacy, Auto, rcx, Byte]                                   ,  [0x0F, 0xAD        ], X, AUTO_SIZE | ENC_MR;
    &[LegacyMemory, Auto, Legacy, Auto, Immediate, Byte]                             ,  [0x0F, 0xAC        ], X, AUTO_SIZE | ENC_MR;
]
"shrx" = [
    &[Legacy, Auto, LegacyMemory, Auto, Legacy, Auto]                                ,  [0x02, 0xF7        ], X, VEX_OP | AUTO_REXW | ENC_MR | PREF_F2, BMI2;
]
"shufpd" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x0F, 0xC6        ], X, PREF_66, SSE2;
]
"shufps" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x0F, 0xC6        ], X, DEFAULT, SSE;
]
"sidt" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0x01        ], 1;
]
"skinit" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xDE  ], X;
]
"sldt" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0x00        ], 0;
    &[Legacy, Auto]                                                                  ,  [0x0F, 0x00        ], 0, AUTO_SIZE;
]
"slwpcb" = [
    &[Legacy, Auto]                                                                  ,  [0x09, 0x12        ], 1, XOP_OP | AUTO_REXW, AMD;
]
"smint" = [
    &[]                                                                              ,  [0x0F, 0x38        ], X, DEFAULT, CYRIX;
]
"smsw" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0x01        ], 4;
    &[Legacy, Auto]                                                                  ,  [0x0F, 0x01        ], 4, AUTO_SIZE;
]
"sqrtpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x51        ], X, PREF_66, SSE2;
]
"sqrtps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x51        ], X, DEFAULT, SSE;
]
"sqrtsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x51        ], X, PREF_F2, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x51        ], X, PREF_F2, SSE2;
]
"sqrtss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x51        ], X, PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x51        ], X, PREF_F3, SSE;
]
"stac" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xCB  ], X;
]
"stc" = [
    &[]                                                                              ,  [0xF9              ], X;
]
"std" = [
    &[]                                                                              ,  [0xFD              ], X;
]
"stgi" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xDC  ], X, DEFAULT, VMX | AMD;
]
"sti" = [
    &[]                                                                              ,  [0xFB              ], X;
]
"stmxcsr" = [
    &[Memory, DWord]                                                                 ,  [0x0F, 0xAE        ], 3, DEFAULT, SSE;
]
"stosb" = [
    &[]                                                                              ,  [0xAA              ], X, REP;
]
"stosd" = [
    &[]                                                                              ,  [0xAB              ], X, REP;
]
"stosq" = [
    &[]                                                                              ,  [0xAB              ], X, WITH_REXW | REP;
]
"stosw" = [
    &[]                                                                              ,  [0xAB              ], X, WORD_SIZE | REP;
]
"str" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0x00        ], 1;
    &[Legacy, Auto]                                                                  ,  [0x0F, 0x00        ], 1, AUTO_SIZE;
]
"sub" = [
    &[rax, Byte, Immediate, Byte]                                                    ,  [0x2C              ], X;
    &[Memory, Byte, Immediate, Byte]                                                 ,  [0x80              ], 5, LOCK;
    &[Memory, Byte, Legacy, Byte]                                                    ,  [0x28              ], X, LOCK | ENC_MR;
    &[Legacy, Byte, Immediate, Byte]                                                 ,  [0x80              ], 5;
    &[Legacy, Byte, Legacy, Byte]                                                    ,  [0x28              ], X, ENC_MR;
    &[Legacy, Byte, LegacyMemory, Byte]                                              ,  [0x2A              ], X;
    &[Legacy, Auto, Immediate, Byte]                                                 ,  [0x83              ], 5, AUTO_SIZE  | EXACT_SIZE;
    &[rax, Auto, Immediate, Auto]                                                    ,  [0x2D              ], X, AUTO_SIZE;
    &[Memory, Auto, Immediate, Auto]                                                 ,  [0x81              ], 5, AUTO_SIZE | LOCK;
    &[Memory, Auto, Immediate, Byte]                                                 ,  [0x83              ], 5, AUTO_SIZE | LOCK;
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x29              ], X, AUTO_SIZE | LOCK | ENC_MR;
    &[Legacy, Auto, Immediate, Auto]                                                 ,  [0x81              ], 5, AUTO_SIZE ;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x29              ], X, AUTO_SIZE | ENC_MR;
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x2B              ], X, AUTO_SIZE;
]
"subpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x5C        ], X, PREF_66, SSE2;
]
"subps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x5C        ], X, DEFAULT, SSE;
]
"subsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x5C        ], X, PREF_F2, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x5C        ], X, PREF_F2, SSE2;
]
"subss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x5C        ], X, PREF_F3, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x5C        ], X, PREF_F3, SSE;
]
"svdc" = [
    &[Memory, PWord, Segment, Word]                                                  ,  [0x0F, 0x78        ], X, ENC_MR | EXACT_SIZE, CYRIX;
]
"svldt" = [
    &[Memory, PWord]                                                                 ,  [0x0F, 0x7A        ], 0, EXACT_SIZE, CYRIX;
]
"svts" = [
    &[Memory, PWord]                                                                 ,  [0x0F, 0x7C        ], 0, EXACT_SIZE, CYRIX;
]
"swapgs" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xF8  ], X;
]
"syscall" = [
    &[]                                                                              ,  [0x0F, 0x05        ], X, DEFAULT, AMD;
]
"sysenter" = [
    &[]                                                                              ,  [0x0F, 0x34        ], X, X86_ONLY;
]
"sysexit" = [
    &[]                                                                              ,  [0x0F, 0x35        ], X, X86_ONLY;
]
"sysret" = [
    &[]                                                                              ,  [0x0F, 0x07        ], X, DEFAULT, AMD;
]
"t1mskc" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x09, 0x01        ], 7, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"test" = [
    &[rax, Byte, Immediate, Byte]                                                    ,  [0xA8              ], X;
    &[Legacy, Byte, Memory, Byte]                                                    ,  [0x84              ], X;
    &[LegacyMemory, Byte, Immediate, Byte]                                           ,  [0xF6              ], 0;
    &[LegacyMemory, Byte, Legacy, Byte]                                              ,  [0x84              ], X, ENC_MR;
    &[rax, Auto, Immediate, Auto]                                                    ,  [0xA9              ], X, AUTO_SIZE;
    &[Legacy, Auto, Memory, Auto]                                                    ,  [0x85              ], X, AUTO_SIZE;
    &[LegacyMemory, Auto, Immediate, Auto]                                           ,  [0xF7              ], 0, AUTO_SIZE;
    &[LegacyMemory, Auto, Legacy, Auto]                                              ,  [0x85              ], X, AUTO_SIZE | ENC_MR;
]
"tzcnt" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x0F, 0xBC        ], X, AUTO_SIZE | PREF_F3, BMI1;
]
"tzmsk" = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x09, 0x01        ], 4, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"ucomisd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x0F, 0x2E        ], X, PREF_66, SSE2;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x2E        ], X, PREF_66, SSE2;
]
"ucomiss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x0F, 0x2E        ], X, DEFAULT, SSE;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x0F, 0x2E        ], X, DEFAULT, SSE;
]
"ud2" = [
    &[]                                                                              ,  [0x0F, 0x0B        ], X;
]
"ud2a" = [
    &[]                                                                              ,  [0x0F, 0x0B        ], X;
]
"unpckhpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x15        ], X, PREF_66, SSE2;
]
"unpckhps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x15        ], X, DEFAULT, SSE;
]
"unpcklpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x14        ], X, PREF_66, SSE2;
]
"unpcklps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x14        ], X, DEFAULT, SSE;
]
"vaddpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x58        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vaddps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x58        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vaddsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0x58        ], X, VEX_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x58        ], X, VEX_OP | PREF_F2, AVX;
]
"vaddss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x01, 0x58        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x58        ], X, VEX_OP | PREF_F3, AVX;
]
"vaddsubpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xD0        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vaddsubps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xD0        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
]
"vaesdec" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x02, 0xDE        ], X, VEX_OP | PREF_66, AVX;
]
"vaesdeclast" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x02, 0xDF        ], X, VEX_OP | PREF_66, AVX;
]
"vaesenc" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x02, 0xDC        ], X, VEX_OP | PREF_66, AVX;
]
"vaesenclast" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x02, 0xDD        ], X, VEX_OP | PREF_66, AVX;
]
"vaesimc" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x02, 0xDB        ], X, VEX_OP | PREF_66, AVX;
]
"vaeskeygenassist" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x03, 0xDF        ], X, VEX_OP | PREF_66, AVX;
]
"vandnpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x55        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vandnps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x55        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vandpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x54        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vandps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x54        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vblendpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, Immediate, Byte]             ,  [0x03, 0x0D        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vblendps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, Immediate, Byte]             ,  [0x03, 0x0C        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vblendvpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x4B        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vblendvps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x4A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vbroadcastf128" = [
    &[AVXRegister, HWord, Memory, OWord]                                             ,  [0x02, 0x1A        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX;
]
"vbroadcasti128" = [
    &[AVXRegister, HWord, Memory, OWord]                                             ,  [0x02, 0x5A        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX2;
]
"vbroadcastsd" = [
    &[AVXRegister, HWord, Memory, QWord]                                             ,  [0x02, 0x19        ], X, VEX_OP | WITH_VEXL | PREF_66, AVX;
    &[AVXRegister, HWord, AVXRegister, OWord]                                        ,  [0x02, 0x19        ], X, VEX_OP | WITH_VEXL | PREF_66, AVX;
]
"vbroadcastss" = [
    &[AVXRegister, Auto, Memory, DWord]                                              ,  [0x02, 0x18        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, OWord]                                         ,  [0x02, 0x18        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vcmpeq_ospd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpeq_osps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpeq_ossd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpeq_osss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpeq_uqpd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x08  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x08  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpeq_uqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x08  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpeq_uqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x08  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x08  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpeq_uqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x08  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x08  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpeq_uspd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x18  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x18  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpeq_usps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x18  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpeq_ussd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x18  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x18  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpeq_usss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x18  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x18  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpeqpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x00  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpeqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x00  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpeqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x00  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x00  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpeqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x00  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x00  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpfalse_oqpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpfalse_oqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpfalse_oqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpfalse_oqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpfalse_ospd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x1B  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpfalse_osps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x1B  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpfalse_ossd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x1B  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x1B  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpfalse_osss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x1B  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x1B  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpfalsepd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x0B  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpfalseps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpfalsesd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpfalsess" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpge_oqpd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x1D  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x1D  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpge_oqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x1D  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpge_oqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x1D  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x1D  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpge_oqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x1D  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x1D  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpge_ospd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpge_osps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpge_ossd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpge_osss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpgepd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpgeps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpgesd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpgess" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpgt_oqpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x1E  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpgt_oqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x1E  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpgt_oqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x1E  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x1E  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpgt_oqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x1E  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x1E  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpgt_ospd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpgt_osps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpgt_ossd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpgt_osss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpgtpd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x0E  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpgtps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpgtsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpgtss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmple_oqpd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x12  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x12  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmple_oqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x12  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmple_oqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x12  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x12  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmple_oqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x12  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x12  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmple_ospd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmple_osps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmple_ossd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmple_osss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmplepd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x02  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpleps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmplesd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpless" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmplt_oqpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x11  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmplt_oqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x11  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmplt_oqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x11  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x11  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmplt_oqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x11  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x11  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmplt_ospd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x01  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmplt_osps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmplt_ossd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmplt_osss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpltpd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x01  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpltps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpltsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpltss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpneq_oqpd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x0C  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x0C  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpneq_oqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0C  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpneq_oqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0C  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0C  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpneq_oqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0C  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0C  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpneq_ospd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x1C  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x1C  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpneq_osps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x1C  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpneq_ossd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x1C  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x1C  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpneq_osss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x1C  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x1C  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpneq_uqpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpneq_uqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpneq_uqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpneq_uqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpneq_uspd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x14  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x14  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpneq_usps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x14  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpneq_ussd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x14  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x14  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpneq_usss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x14  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x14  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpneqpd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x04  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpneqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpneqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpneqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpnge_uqpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x19  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpnge_uqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x19  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnge_uqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x19  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x19  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpnge_uqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x19  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x19  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpnge_uspd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpnge_usps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnge_ussd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpnge_usss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpngepd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x09  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpngeps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpngesd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpngess" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpngt_uqpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x1A  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpngt_uqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x1A  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpngt_uqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x1A  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x1A  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpngt_uqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x1A  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x1A  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpngt_uspd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpngt_usps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpngt_ussd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpngt_usss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpngtpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpngtps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpngtsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpngtss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpnle_uqpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x16  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpnle_uqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x16  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnle_uqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x16  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x16  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpnle_uqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x16  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x16  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpnle_uspd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x06  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpnle_usps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnle_ussd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpnle_usss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpnlepd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpnleps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnlesd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpnless" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpnlt_uqpd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x15  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x15  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpnlt_uqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x15  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnlt_uqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x15  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x15  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpnlt_uqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x15  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x15  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpnlt_uspd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpnlt_usps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnlt_ussd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpnlt_usss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpnltpd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x05  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpnltps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnltsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpnltss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpord_qpd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x07  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpord_qps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpord_qsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpord_qss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpord_spd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x17  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x17  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpord_sps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x17  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpord_ssd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x17  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x17  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpord_sss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x17  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x17  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpordpd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x07  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpordps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpordsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpordss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmppd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, Immediate, Byte]             ,  [0x01, 0xC2        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vcmpps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, Immediate, Byte]             ,  [0x01, 0xC2        ], X, VEX_OP | AUTO_VEXL | ENC_MR, AVX;
]
"vcmpsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord, Immediate, Byte]        ,  [0x01, 0xC2        ], X, VEX_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]   ,  [0x01, 0xC2        ], X, VEX_OP | PREF_F2, AVX;
]
"vcmpss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord, Immediate, Byte]        ,  [0x01, 0xC2        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]   ,  [0x01, 0xC2        ], X, VEX_OP | PREF_F3, AVX;
]
"vcmptrue_uqpd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x0F  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmptrue_uqps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmptrue_uqsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmptrue_uqss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmptrue_uspd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x1F  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmptrue_usps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x1F  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmptrue_ussd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x1F  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x1F  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmptrue_usss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x1F  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x1F  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmptruepd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmptrueps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmptruesd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmptruess" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpunord_qpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpunord_qps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpunord_qsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpunord_qss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpunord_spd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x13  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpunord_sps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x13  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpunord_ssd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x13  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x13  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpunord_sss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x13  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x13  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpunordpd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x01, 0xC2, 0x03  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpunordps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpunordsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpunordss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcomisd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x01, 0x2F        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x01, 0x2F        ], X, VEX_OP | PREF_66, AVX;
]
"vcomiss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x01, 0x2F        ], X, VEX_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x01, 0x2F        ], X, VEX_OP, AVX;
]
"vcvtdq2pd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x01, 0xE6        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, Auto, Word, OWord]                                                ,  [0x01, 0xE6        ], X, VEX_OP | AUTO_VEXL | PREF_F3, AVX;
]
"vcvtdq2ps" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x5B        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vcvtpd2dq" = [
    &[AVXRegister, OWord, Memory, Auto]                                              ,  [0x01, 0xE6        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, Auto]                                         ,  [0x01, 0xE6        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
]
"vcvtpd2ps" = [
    &[AVXRegister, OWord, Memory, Auto]                                              ,  [0x01, 0x5A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, Auto]                                         ,  [0x01, 0x5A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vcvtph2ps" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x02, 0x13        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, Auto, Word, OWord]                                                ,  [0x02, 0x13        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vcvtps2dq" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x5B        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vcvtps2pd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x01, 0x5A        ], X, VEX_OP, AVX;
    &[AVXRegister, Auto, Word, OWord]                                                ,  [0x01, 0x5A        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vcvtps2ph" = [
    &[Memory, QWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x03, 0x1D        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    &[Word, OWord, AVXRegister, Auto, Immediate, Byte]                               ,  [0x03, 0x1D        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vcvtsd2si" = [
    &[Legacy, Auto, Memory, QWord]                                                   ,  [0x01, 0x2D        ], X, VEX_OP | AUTO_REXW | PREF_F2, AVX;
    &[Legacy, Auto, AVXRegister, OWord]                                              ,  [0x01, 0x2D        ], X, VEX_OP | AUTO_REXW | PREF_F2, AVX;
]
"vcvtsd2ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0x5A        ], X, VEX_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x5A        ], X, VEX_OP | PREF_F2, AVX;
]
"vcvtsi2sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, LegacyMemory, Auto]                    ,  [0x01, 0x2A        ], X, VEX_OP | AUTO_REXW | PREF_F2, AVX;
]
"vcvtsi2ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, LegacyMemory, Auto]                    ,  [0x01, 0x2A        ], X, VEX_OP | AUTO_REXW | PREF_F3, AVX;
]
"vcvtss2sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x01, 0x5A        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x5A        ], X, VEX_OP | PREF_F3, AVX;
]
"vcvtss2si" = [
    &[Legacy, Auto, Memory, DWord]                                                   ,  [0x01, 0x2D        ], X, VEX_OP | AUTO_REXW | PREF_F3, AVX;
    &[Legacy, Auto, AVXRegister, OWord]                                              ,  [0x01, 0x2D        ], X, VEX_OP | AUTO_REXW | PREF_F3, AVX;
]
"vcvttpd2dq" = [
    &[AVXRegister, OWord, Memory, Auto]                                              ,  [0x01, 0xE6        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, Auto]                                         ,  [0x01, 0xE6        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vcvttps2dq" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x5B        ], X, VEX_OP | AUTO_VEXL | PREF_F3, AVX;
]
"vcvttsd2si" = [
    &[Legacy, Auto, Memory, QWord]                                                   ,  [0x01, 0x2C        ], X, VEX_OP | AUTO_REXW | PREF_F2, AVX;
    &[Legacy, Auto, AVXRegister, OWord]                                              ,  [0x01, 0x2C        ], X, VEX_OP | AUTO_REXW | PREF_F2, AVX;
]
"vcvttss2si" = [
    &[Legacy, Auto, Memory, DWord]                                                   ,  [0x01, 0x2C        ], X, VEX_OP | AUTO_REXW | PREF_F3, AVX;
    &[Legacy, Auto, AVXRegister, OWord]                                              ,  [0x01, 0x2C        ], X, VEX_OP | AUTO_REXW | PREF_F3, AVX;
]
"vdivpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x5E        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vdivps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x5E        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vdivsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0x5E        ], X, VEX_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x5E        ], X, VEX_OP | PREF_F2, AVX;
]
"vdivss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x01, 0x5E        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x5E        ], X, VEX_OP | PREF_F3, AVX;
]
"vdppd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, Immediate, Byte]          ,  [0x03, 0x41        ], X, VEX_OP | PREF_66, AVX;
]
"vdpps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, Immediate, Byte]             ,  [0x03, 0x40        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"verr" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0x00        ], 4;
    &[Legacy, Word]                                                                  ,  [0x0F, 0x00        ], 4;
]
"verw" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0x00        ], 5;
    &[Legacy, Word]                                                                  ,  [0x0F, 0x00        ], 5;
]
"vextractf128" = [
    &[Word, OWord, AVXRegister, HWord, Immediate, Byte]                              ,  [0x03, 0x19        ], X, WITH_VEXL | VEX_OP | ENC_MR | PREF_66, AVX;
]
"vextracti128" = [
    &[Word, OWord, AVXRegister, HWord, Immediate, Byte]                              ,  [0x03, 0x39        ], X, WITH_VEXL | VEX_OP | ENC_MR | PREF_66, AVX2;
]
"vextractps" = [
    &[LegacyMemory, DWord, AVXRegister, OWord, Immediate, Byte]                      ,  [0x03, 0x17        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vfmadd123pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xA8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd123ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xA8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd123sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xA9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xA9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmadd123ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xA9        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xA9        ], X, VEX_OP | PREF_66, FMA;
]
"vfmadd132pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x98        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd132ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x98        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd132sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0x99        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x99        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmadd132ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0x99        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x99        ], X, VEX_OP | PREF_66, FMA;
]
"vfmadd213pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xA8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd213ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xA8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd213sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xA9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xA9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmadd213ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xA9        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xA9        ], X, VEX_OP | PREF_66, FMA;
]
"vfmadd231pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xB8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd231ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xB8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd231sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xB9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xB9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmadd231ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xB9        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xB9        ], X, VEX_OP | PREF_66, FMA;
]
"vfmadd312pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x98        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd312ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x98        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd312sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0x99        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x99        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmadd312ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0x99        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x99        ], X, VEX_OP | PREF_66, FMA;
]
"vfmadd321pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xB8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd321ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xB8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd321sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xB9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xB9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmadd321ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xB9        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xB9        ], X, VEX_OP | PREF_66, FMA;
]
"vfmaddpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, AVXRegister, Auto, Word, Auto]           ,  [0x03, 0x69        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x69        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
]
"vfmaddps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, AVXRegister, Auto, Word, Auto]           ,  [0x03, 0x68        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x68        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
]
"vfmaddsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord, AVXRegister, OWord]     ,  [0x03, 0x6B        ], X, VEX_OP | PREF_66, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]     ,  [0x03, 0x6B        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord],  [0x03, 0x6B        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
]
"vfmaddss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord, AVXRegister, OWord]     ,  [0x03, 0x6A        ], X, VEX_OP | PREF_66, SSE5 | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]     ,  [0x03, 0x6A        ], X, VEX_OP | WITH_REXW | PREF_66, SSE5 | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord],  [0x03, 0x6A        ], X, VEX_OP | WITH_REXW | PREF_66, SSE5 | AMD;
]
"vfmaddsub123pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xA6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub123ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xA6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub132pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x96        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub132ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x96        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub213pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xA6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub213ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xA6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub231pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xB6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub231ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xB6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub312pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x96        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub312ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x96        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub321pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xB6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub321ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xB6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsubpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, AVXRegister, Auto, Word, Auto]           ,  [0x03, 0x5D        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x5D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfmaddsubps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, AVXRegister, Auto, Word, Auto]           ,  [0x03, 0x5C        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x5C        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
]
"vfmsub123pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xAA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub123ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xAA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub123sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xAB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xAB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmsub123ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xAB        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xAB        ], X, VEX_OP | PREF_66, FMA;
]
"vfmsub132pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x9A        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub132ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x9A        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub132sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0x9B        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x9B        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmsub132ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0x9B        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x9B        ], X, VEX_OP | PREF_66, FMA;
]
"vfmsub213pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xAA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub213ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xAA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub213sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xAB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xAB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmsub213ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xAB        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xAB        ], X, VEX_OP | PREF_66, FMA;
]
"vfmsub231pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xBA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub231ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xBA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub231sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xBB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xBB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmsub231ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xBB        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xBB        ], X, VEX_OP | PREF_66, FMA;
]
"vfmsub312pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x9A        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub312ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x9A        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub312sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0x9B        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x9B        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmsub312ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0x9B        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x9B        ], X, VEX_OP | PREF_66, FMA;
]
"vfmsub321pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xBA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub321ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xBA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub321sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xBB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xBB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmsub321ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xBB        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xBB        ], X, VEX_OP | PREF_66, FMA;
]
"vfmsubadd123pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xA7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd123ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xA7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd132pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x97        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd132ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x97        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd213pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xA7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd213ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xA7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd231pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xB7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd231ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xB7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd312pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x97        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd312ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x97        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd321pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xB7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd321ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xB7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubaddpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, AVXRegister, Auto, Word, Auto]           ,  [0x03, 0x5F        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x5F        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfmsubaddps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, AVXRegister, Auto, Word, Auto]           ,  [0x03, 0x5E        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x5E        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfmsubpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, AVXRegister, Auto, Word, Auto]           ,  [0x03, 0x6D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x6D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfmsubps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, AVXRegister, Auto, Word, Auto]           ,  [0x03, 0x6C        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x6C        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
]
"vfmsubsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord, AVXRegister, OWord]     ,  [0x03, 0x6F        ], X, VEX_OP | PREF_66, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]     ,  [0x03, 0x6F        ], X, VEX_OP | WITH_REXW | PREF_66, SSE5 | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord],  [0x03, 0x6F        ], X, VEX_OP | WITH_REXW | PREF_66, SSE5 | AMD;
]
"vfmsubss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord, AVXRegister, OWord]     ,  [0x03, 0x6E        ], X, VEX_OP | PREF_66, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]     ,  [0x03, 0x6E        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord],  [0x03, 0x6E        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
]
"vfnmadd123pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xAC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd123ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xAC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd123sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xAD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xAD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmadd123ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xAD        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xAD        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmadd132pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x9C        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd132ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x9C        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd132sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0x9D        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x9D        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmadd132ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0x9D        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x9D        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmadd213pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xAC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd213ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xAC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd213sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xAD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xAD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmadd213ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xAD        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xAD        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmadd231pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xBC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd231ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xBC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd231sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xBD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xBD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmadd231ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xBD        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xBD        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmadd312pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x9C        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd312ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x9C        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd312sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0x9D        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x9D        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmadd312ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0x9D        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x9D        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmadd321pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xBC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd321ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xBC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd321sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xBD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xBD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmadd321ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xBD        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xBD        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmaddpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, AVXRegister, Auto, Word, Auto]           ,  [0x03, 0x79        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x79        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfnmaddps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, AVXRegister, Auto, Word, Auto]           ,  [0x03, 0x78        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x78        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
]
"vfnmaddsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord, AVXRegister, OWord]     ,  [0x03, 0x7B        ], X, VEX_OP | PREF_66, SSE5 | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]     ,  [0x03, 0x7B        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord],  [0x03, 0x7B        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
]
"vfnmaddss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord, AVXRegister, OWord]     ,  [0x03, 0x7A        ], X, VEX_OP | PREF_66, SSE5 | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]     ,  [0x03, 0x7A        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord],  [0x03, 0x7A        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
]
"vfnmsub123pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xAE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub123ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xAE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub123sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xAF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xAF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmsub123ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xAF        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xAF        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmsub132pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x9E        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub132ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x9E        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub132sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0x9F        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x9F        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmsub132ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0x9F        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x9F        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmsub213pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xAE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub213ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xAE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub213sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xAF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xAF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmsub213ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xAF        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xAF        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmsub231pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xBE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub231ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xBE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub231sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xBF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xBF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmsub231ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xBF        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xBF        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmsub312pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x9E        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub312ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x9E        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub312sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0x9F        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x9F        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmsub312ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0x9F        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0x9F        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmsub321pd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xBE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub321ps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0xBE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub321sd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x02, 0xBF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xBF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmsub321ss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x02, 0xBF        ], X, VEX_OP | PREF_66, FMA;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x02, 0xBF        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmsubpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, AVXRegister, Auto, Word, Auto]           ,  [0x03, 0x7D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x7D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfnmsubps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, AVXRegister, Auto, Word, Auto]           ,  [0x03, 0x7C        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x7C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfnmsubsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord, AVXRegister, OWord]     ,  [0x03, 0x7F        ], X, VEX_OP | PREF_66, SSE5 | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]     ,  [0x03, 0x7F        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord],  [0x03, 0x7F        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
]
"vfnmsubss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord, AVXRegister, OWord]     ,  [0x03, 0x7E        ], X, VEX_OP | PREF_66, SSE5 | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]     ,  [0x03, 0x7E        ], X, VEX_OP | WITH_REXW | PREF_66, SSE5 | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord],  [0x03, 0x7E        ], X, VEX_OP | WITH_REXW | PREF_66, SSE5 | AMD;
]
"vfrczpd" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x09, 0x81        ], X, XOP_OP | AUTO_VEXL, SSE5 | AMD;
]
"vfrczps" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x09, 0x80        ], X, XOP_OP | AUTO_VEXL, AMD | SSE5;
]
"vfrczsd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x09, 0x83        ], X, XOP_OP, SSE5 | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x09, 0x83        ], X, XOP_OP, SSE5 | AMD;
]
"vfrczss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x09, 0x82        ], X, XOP_OP, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x09, 0x82        ], X, XOP_OP, AMD | SSE5;
]
"vgatherdpd" = [
    &[AVXRegister, Auto, VSIB64, OWord, AVXRegister, Auto]                           ,  [0x02, 0x92        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vgatherdps" = [
    &[AVXRegister, Auto, VSIB32, Auto, AVXRegister, Auto]                            ,  [0x02, 0x92        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vgatherqpd" = [
    &[AVXRegister, Auto, VSIB64, Auto, AVXRegister, Auto]                            ,  [0x02, 0x93        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vgatherqps" = [
    &[AVXRegister, OWord, VSIB32, Auto, AVXRegister, OWord]                          ,  [0x02, 0x93        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vhaddpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x7C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vhaddps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x7C        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
]
"vhsubpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x7D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vhsubps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x7D        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
]
"vinsertf128" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, OWord, Immediate, Byte]          ,  [0x03, 0x18        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX;
]
"vinserti128" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, OWord, Immediate, Byte]          ,  [0x03, 0x38        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX2;
]
"vinsertps" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord, Immediate, Byte]        ,  [0x03, 0x21        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]   ,  [0x03, 0x21        ], X, VEX_OP | PREF_66, AVX;
]
"vlddqu" = [
    &[AVXRegister, Auto, Memory, Auto]                                               ,  [0x01, 0xF0        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
]
"vldmxcsr" = [
    &[Memory, DWord]                                                                 ,  [0x01, 0xAE        ], 2, VEX_OP, AVX;
]
"vldqqu" = [
    &[AVXRegister, HWord, Memory, HWord]                                             ,  [0x01, 0xF0        ], X, WITH_VEXL | VEX_OP | PREF_F2, AVX;
]
"vmaskmovdqu" = [
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x01, 0xF7        ], X, VEX_OP | PREF_66, AVX;
]
"vmaskmovpd" = [
    &[Memory, Auto, AVXRegister, Auto, AVXRegister, Auto]                            ,  [0x02, 0x2F        ], X, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, Auto, Memory, Auto]                            ,  [0x02, 0x2D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vmaskmovps" = [
    &[Memory, Auto, AVXRegister, Auto, AVXRegister, Auto]                            ,  [0x02, 0x2E        ], X, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, Auto, Memory, Auto]                            ,  [0x02, 0x2C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vmaxpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x5F        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vmaxps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x5F        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vmaxsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0x5F        ], X, VEX_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x5F        ], X, VEX_OP | PREF_F2, AVX;
]
"vmaxss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x01, 0x5F        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x5F        ], X, VEX_OP | PREF_F3, AVX;
]
"vmcall" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xC1  ], X, DEFAULT, VMX;
]
"vmclear" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xC7        ], 6, PREF_66, VMX;
]
"vmfunc" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xD4  ], X, DEFAULT, VMX;
]
"vminpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x5D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vminps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x5D        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vminsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0x5D        ], X, VEX_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x5D        ], X, VEX_OP | PREF_F2, AVX;
]
"vminss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x01, 0x5D        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x5D        ], X, VEX_OP | PREF_F3, AVX;
]
"vmlaunch" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xC2  ], X, DEFAULT, VMX;
]
"vmload" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xDA  ], X, DEFAULT, AMD | VMX;
]
"vmmcall" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xD9  ], X, DEFAULT, AMD | VMX;
]
"vmovapd" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x28        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    &[Word, HWord, AVXRegister, HWord]                                               ,  [0x01, 0x29        ], X, VEX_OP | WITH_VEXL | ENC_MR | PREF_66, AVX;
    &[Word, OWord, AVXRegister, OWord]                                               ,  [0x01, 0x29        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovaps" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x28        ], X, VEX_OP | AUTO_VEXL, AVX;
    &[Word, HWord, AVXRegister, HWord]                                               ,  [0x01, 0x29        ], X, VEX_OP | WITH_VEXL | ENC_MR, AVX;
    &[Word, OWord, AVXRegister, OWord]                                               ,  [0x01, 0x29        ], X, VEX_OP | ENC_MR, AVX;
]
"vmovd" = [
    &[AVXRegister, OWord, LegacyMemory, DWord]                                       ,  [0x01, 0x6E        ], X, VEX_OP | PREF_66, AVX;
    &[LegacyMemory, DWord, AVXRegister, OWord]                                       ,  [0x01, 0x7E        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovddup" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x12        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x01, 0x12        ], X, VEX_OP | PREF_F2, AVX;
]
"vmovdqa" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x6F        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    &[Word, HWord, AVXRegister, HWord]                                               ,  [0x01, 0x7F        ], X, VEX_OP | WITH_VEXL | ENC_MR | PREF_66, AVX;
    &[Word, OWord, AVXRegister, OWord]                                               ,  [0x01, 0x7F        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovdqu" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x6F        ], X, VEX_OP | AUTO_VEXL | PREF_F3, AVX;
    &[Word, HWord, AVXRegister, HWord]                                               ,  [0x01, 0x7F        ], X, VEX_OP | WITH_VEXL | ENC_MR | PREF_F3, AVX;
    &[Word, OWord, AVXRegister, OWord]                                               ,  [0x01, 0x7F        ], X, VEX_OP | ENC_MR | PREF_F3, AVX;
]
"vmovhlps" = [
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x12        ], X, VEX_OP, AVX;
]
"vmovhpd" = [
    &[Memory, QWord, AVXRegister, OWord]                                             ,  [0x01, 0x17        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0x16        ], X, VEX_OP | PREF_66, AVX;
]
"vmovhps" = [
    &[Memory, QWord, AVXRegister, OWord]                                             ,  [0x01, 0x17        ], X, VEX_OP | ENC_MR, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0x16        ], X, VEX_OP, AVX;
]
"vmovlhps" = [
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x16        ], X, VEX_OP, AVX;
]
"vmovlpd" = [
    &[Memory, QWord, AVXRegister, OWord]                                             ,  [0x01, 0x13        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0x12        ], X, VEX_OP | PREF_66, AVX;
]
"vmovlps" = [
    &[Memory, QWord, AVXRegister, OWord]                                             ,  [0x01, 0x13        ], X, VEX_OP | ENC_MR, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0x12        ], X, VEX_OP, AVX;
]
"vmovmskpd" = [
    &[Legacy, Auto, AVXRegister, Auto]                                               ,  [0x01, 0x50        ], X, VEX_OP | PREF_66, AVX;
]
"vmovmskps" = [
    &[Legacy, Auto, AVXRegister, Auto]                                               ,  [0x01, 0x50        ], X, VEX_OP, AVX;
]
"vmovntdq" = [
    &[Memory, Auto, AVXRegister, Auto]                                               ,  [0x01, 0xE7        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vmovntdqa" = [
    &[AVXRegister, Auto, Memory, Auto]                                               ,  [0x02, 0x2A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vmovntpd" = [
    &[Memory, Auto, AVXRegister, Auto]                                               ,  [0x01, 0x2B        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vmovntps" = [
    &[Memory, Auto, AVXRegister, Auto]                                               ,  [0x01, 0x2B        ], X, VEX_OP | AUTO_VEXL | ENC_MR, AVX;
]
"vmovntqq" = [
    &[Memory, HWord, AVXRegister, HWord]                                             ,  [0x01, 0xE7        ], X, WITH_VEXL | VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovq" = [
    &[Memory, QWord, AVXRegister, OWord]                                             ,  [0x01, 0xD6        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x01, 0x7E        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, LegacyMemory, QWord]                                       ,  [0x01, 0x6E        ], X, WITH_REXW | VEX_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x01, 0x7E        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x01, 0xD6        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    &[LegacyMemory, QWord, AVXRegister, OWord]                                       ,  [0x01, 0x7E        ], X, WITH_REXW | VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovqqa" = [
    &[AVXRegister, HWord, Word, HWord]                                               ,  [0x01, 0x6F        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX;
    &[Word, HWord, AVXRegister, HWord]                                               ,  [0x01, 0x7F        ], X, WITH_VEXL | VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovqqu" = [
    &[AVXRegister, HWord, Word, HWord]                                               ,  [0x01, 0x6F        ], X, WITH_VEXL | VEX_OP | PREF_F3, AVX;
    &[Word, HWord, AVXRegister, HWord]                                               ,  [0x01, 0x7F        ], X, WITH_VEXL | VEX_OP | ENC_MR | PREF_F3, AVX;
]
"vmovsd" = [
    &[Memory, QWord, AVXRegister, OWord]                                             ,  [0x01, 0x11        ], X, VEX_OP | ENC_MR | PREF_F2, AVX;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x01, 0x10        ], X, VEX_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x10        ], X, VEX_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x11        ], X, VEX_OP | ENC_VM | PREF_F2, AVX;
]
"vmovshdup" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x16        ], X, VEX_OP | AUTO_VEXL | PREF_F3, AVX;
]
"vmovsldup" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x12        ], X, VEX_OP | AUTO_VEXL | PREF_F3, AVX;
]
"vmovss" = [
    &[Memory, DWord, AVXRegister, OWord]                                             ,  [0x01, 0x11        ], X, VEX_OP | ENC_MR | PREF_F3, AVX;
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x01, 0x10        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x10        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x11        ], X, VEX_OP | ENC_VM | PREF_F3, AVX;
]
"vmovupd" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x10        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    &[Word, HWord, AVXRegister, HWord]                                               ,  [0x01, 0x11        ], X, VEX_OP | WITH_VEXL | ENC_MR | PREF_66, AVX;
    &[Word, OWord, AVXRegister, OWord]                                               ,  [0x01, 0x11        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovups" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x10        ], X, VEX_OP | AUTO_VEXL, AVX;
    &[Word, HWord, AVXRegister, HWord]                                               ,  [0x01, 0x11        ], X, VEX_OP | WITH_VEXL | ENC_MR, AVX;
    &[Word, OWord, AVXRegister, OWord]                                               ,  [0x01, 0x11        ], X, VEX_OP | ENC_MR, AVX;
]
"vmpsadbw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, Immediate, Byte]             ,  [0x03, 0x42        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vmptrld" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xC7        ], 6, DEFAULT, VMX;
]
"vmptrst" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xC7        ], 7, DEFAULT, VMX;
]
"vmread" = [
    &[LegacyMemory, QWord, Legacy, QWord]                                            ,  [0x0F, 0x78        ], X, ENC_MR, VMX;
]
"vmresume" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xC3  ], X, DEFAULT, VMX;
]
"vmrun" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xD8  ], X, DEFAULT, AMD | VMX;
]
"vmsave" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xDB  ], X, DEFAULT, VMX | AMD;
]
"vmulpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x59        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vmulps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x59        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vmulsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0x59        ], X, VEX_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x59        ], X, VEX_OP | PREF_F2, AVX;
]
"vmulss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x01, 0x59        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x59        ], X, VEX_OP | PREF_F3, AVX;
]
"vmwrite" = [
    &[Legacy, QWord, LegacyMemory, QWord]                                            ,  [0x0F, 0x79        ], X, DEFAULT, VMX;
]
"vmxoff" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xC4  ], X, DEFAULT, VMX;
]
"vmxon" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xC7        ], 6, PREF_F3, VMX;
]
"vorpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x56        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vorps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x56        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vpabsb" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x02, 0x1C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpabsd" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x02, 0x1E        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpabsw" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x02, 0x1D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpackssdw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x6B        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpacksswb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x63        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpackusdw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x2B        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpackuswb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x67        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xFC        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xFE        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xD4        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddsb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xEC        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddsw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xED        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddusb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xDC        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddusw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xDD        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xFD        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpalignr" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, Immediate, Byte]             ,  [0x03, 0x0F        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vpand" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xDB        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpandn" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xDF        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpavgb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xE0        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpavgw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xE3        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpblendd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, Immediate, Byte]             ,  [0x03, 0x02        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vpblendvb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x03, 0x4C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpblendw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, Immediate, Byte]             ,  [0x03, 0x0E        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vpbroadcastb" = [
    &[AVXRegister, Auto, Memory, Byte]                                               ,  [0x02, 0x78        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
    &[AVXRegister, Auto, AVXRegister, OWord]                                         ,  [0x02, 0x78        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpbroadcastd" = [
    &[AVXRegister, Auto, Memory, DWord]                                              ,  [0x02, 0x58        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
    &[AVXRegister, Auto, AVXRegister, OWord]                                         ,  [0x02, 0x58        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpbroadcastq" = [
    &[AVXRegister, HWord, Memory, QWord]                                             ,  [0x02, 0x59        ], X, VEX_OP | WITH_VEXL | PREF_66, AVX2;
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x02, 0x59        ], X, VEX_OP | PREF_66, AVX2;
    &[AVXRegister, Auto, AVXRegister, OWord]                                         ,  [0x02, 0x59        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpbroadcastw" = [
    &[AVXRegister, Auto, Memory, Word]                                               ,  [0x02, 0x79        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
    &[AVXRegister, Auto, AVXRegister, OWord]                                         ,  [0x02, 0x79        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpclmulhqhqdq" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x03, 0x44, 0x11  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vpclmulhqlqdq" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x03, 0x44, 0x01  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vpclmullqhqdq" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x03, 0x44, 0x10  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vpclmullqlqdq" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x03, 0x44, 0x00  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vpclmulqdq" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, Immediate, Byte]          ,  [0x03, 0x44        ], X, VEX_OP | PREF_66, AVX;
]
"vpcmov" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, AVXRegister, Auto]           ,  [0x08, 0xA2        ], X, XOP_OP | AUTO_VEXL, SSE5 | AMD;
    &[AVXRegister, Auto, AVXRegister, Auto, AVXRegister, Auto, Word, Auto]           ,  [0x08, 0xA2        ], X, XOP_OP | AUTO_VEXL, AMD | SSE5;
]
"vpcmpeqb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x74        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpeqd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x76        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpeqq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x29        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpeqw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x75        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpestri" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x03, 0x61        ], X, VEX_OP | PREF_66, AVX;
]
"vpcmpestrm" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x03, 0x60        ], X, VEX_OP | PREF_66, AVX;
]
"vpcmpgtb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x64        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpgtd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x66        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpgtq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x37        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpgtw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x65        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpistri" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x03, 0x63        ], X, VEX_OP | PREF_66, AVX;
]
"vpcmpistrm" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x03, 0x62        ], X, VEX_OP | PREF_66, AVX;
]
"vpcomb" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, Immediate, Byte]          ,  [0x08, 0xCC        ], X, XOP_OP, AMD | SSE5;
]
"vpcomd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, Immediate, Byte]          ,  [0x08, 0xCE        ], X, XOP_OP, AMD | SSE5;
]
"vpcomq" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, Immediate, Byte]          ,  [0x08, 0xCF        ], X, XOP_OP, SSE5 | AMD;
]
"vpcomub" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, Immediate, Byte]          ,  [0x08, 0xEC        ], X, XOP_OP, AMD | SSE5;
]
"vpcomud" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, Immediate, Byte]          ,  [0x08, 0xEE        ], X, XOP_OP, SSE5 | AMD;
]
"vpcomuq" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, Immediate, Byte]          ,  [0x08, 0xEF        ], X, XOP_OP, AMD | SSE5;
]
"vpcomuw" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, Immediate, Byte]          ,  [0x08, 0xED        ], X, XOP_OP, SSE5 | AMD;
]
"vpcomw" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, Immediate, Byte]          ,  [0x08, 0xCD        ], X, XOP_OP, AMD | SSE5;
]
"vperm2f128" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord, Immediate, Byte]          ,  [0x03, 0x06        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX;
]
"vperm2i128" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord, Immediate, Byte]          ,  [0x03, 0x46        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX2;
]
"vpermd" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x02, 0x36        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX2;
]
"vpermilpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x0D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    &[AVXRegister, Auto, Word, Auto, Immediate, Byte]                                ,  [0x03, 0x05        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpermilps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x0C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    &[AVXRegister, Auto, Word, Auto, Immediate, Byte]                                ,  [0x03, 0x04        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpermpd" = [
    &[AVXRegister, HWord, Word, HWord, Immediate, Byte]                              ,  [0x03, 0x01        ], X, WITH_VEXL | WITH_REXW | VEX_OP | PREF_66, AVX2;
]
"vpermps" = [
    &[AVXRegister, HWord, AVXRegister, HWord, Word, HWord]                           ,  [0x02, 0x16        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX2;
]
"vpermq" = [
    &[AVXRegister, HWord, Word, HWord, Immediate, Byte]                              ,  [0x03, 0x00        ], X, WITH_VEXL | WITH_REXW | VEX_OP | PREF_66, AVX2;
]
"vpextrb" = [
    &[Memory, Byte, AVXRegister, OWord, Immediate, Byte]                             ,  [0x03, 0x14        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    &[Legacy, DWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x03, 0x14        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    &[Legacy, QWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x03, 0x14        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vpextrd" = [
    &[Legacy, QWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x03, 0x16        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    &[LegacyMemory, DWord, AVXRegister, OWord, Immediate, Byte]                      ,  [0x03, 0x16        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vpextrq" = [
    &[LegacyMemory, QWord, AVXRegister, OWord, Immediate, Byte]                      ,  [0x03, 0x16        ], X, WITH_REXW | VEX_OP | ENC_MR | PREF_66, AVX;
]
"vpextrw" = [
    &[Memory, Word, AVXRegister, OWord, Immediate, Byte]                             ,  [0x03, 0x15        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    &[Legacy, DWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x01, 0xC5        ], X, VEX_OP | PREF_66, AVX;
    &[Legacy, DWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x03, 0x15        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    &[Legacy, QWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x01, 0xC5        ], X, VEX_OP | PREF_66, AVX;
    &[Legacy, QWord, AVXRegister, OWord, Immediate, Byte]                            ,  [0x03, 0x15        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vpgatherdd" = [
    &[AVXRegister, Auto, VSIB32, Auto, AVXRegister, Auto]                            ,  [0x02, 0x90        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vpgatherdq" = [
    &[AVXRegister, Auto, VSIB64, OWord, AVXRegister, Auto]                           ,  [0x02, 0x90        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vpgatherqd" = [
    &[AVXRegister, OWord, VSIB32, Auto, AVXRegister, OWord]                          ,  [0x02, 0x91        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vpgatherqq" = [
    &[AVXRegister, Auto, VSIB64, Auto, AVXRegister, Auto]                            ,  [0x02, 0x91        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vphaddbd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xC2        ], X, XOP_OP, SSE5 | AMD;
]
"vphaddbq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xC3        ], X, XOP_OP, SSE5 | AMD;
]
"vphaddbw" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xC1        ], X, XOP_OP, SSE5 | AMD;
]
"vphaddd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x02        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vphadddq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xCB        ], X, XOP_OP, SSE5 | AMD;
]
"vphaddsw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x03        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vphaddubd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xD2        ], X, XOP_OP, SSE5 | AMD;
]
"vphaddubq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xD3        ], X, XOP_OP, AMD | SSE5;
]
"vphaddubw" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xD1        ], X, XOP_OP, SSE5 | AMD;
]
"vphaddudq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xDB        ], X, XOP_OP, AMD | SSE5;
]
"vphadduwd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xD6        ], X, XOP_OP, AMD | SSE5;
]
"vphadduwq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xD7        ], X, XOP_OP, AMD | SSE5;
]
"vphaddw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x01        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vphaddwd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xC6        ], X, XOP_OP, AMD | SSE5;
]
"vphaddwq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xC7        ], X, XOP_OP, AMD | SSE5;
]
"vphminposuw" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x02, 0x41        ], X, VEX_OP | PREF_66, AVX;
]
"vphsubbw" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xE1        ], X, XOP_OP, SSE5 | AMD;
]
"vphsubd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x06        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vphsubdq" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xE3        ], X, XOP_OP, SSE5 | AMD;
]
"vphsubsw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x07        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vphsubw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x05        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vphsubwd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x09, 0xE2        ], X, XOP_OP, SSE5 | AMD;
]
"vpinsrb" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Legacy, DWord, Immediate, Byte]        ,  [0x03, 0x20        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, LegacyMemory, Byte, Immediate, Byte]   ,  [0x03, 0x20        ], X, VEX_OP | PREF_66, AVX;
]
"vpinsrd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, LegacyMemory, DWord, Immediate, Byte]  ,  [0x03, 0x22        ], X, VEX_OP | PREF_66, AVX;
]
"vpinsrq" = [
    &[AVXRegister, OWord, AVXRegister, OWord, LegacyMemory, QWord, Immediate, Byte]  ,  [0x03, 0x22        ], X, VEX_OP | WITH_REXW | PREF_66, AVX;
]
"vpinsrw" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Legacy, DWord, Immediate, Byte]        ,  [0x01, 0xC4        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, LegacyMemory, Word, Immediate, Byte]   ,  [0x01, 0xC4        ], X, VEX_OP | PREF_66, AVX;
]
"vpmacsdd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, AVXRegister, OWord]       ,  [0x08, 0x9E        ], X, XOP_OP, AMD | SSE5;
]
"vpmacsdqh" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, AVXRegister, OWord]       ,  [0x08, 0x9F        ], X, XOP_OP, SSE5 | AMD;
]
"vpmacsdql" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, AVXRegister, OWord]       ,  [0x08, 0x97        ], X, XOP_OP, AMD | SSE5;
]
"vpmacssdd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, AVXRegister, OWord]       ,  [0x08, 0x8E        ], X, XOP_OP, SSE5 | AMD;
]
"vpmacssdqh" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, AVXRegister, OWord]       ,  [0x08, 0x8F        ], X, XOP_OP, AMD | SSE5;
]
"vpmacssdql" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, AVXRegister, OWord]       ,  [0x08, 0x87        ], X, XOP_OP, SSE5 | AMD;
]
"vpmacsswd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, AVXRegister, OWord]       ,  [0x08, 0x86        ], X, XOP_OP, AMD | SSE5;
]
"vpmacssww" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, AVXRegister, OWord]       ,  [0x08, 0x85        ], X, XOP_OP, AMD | SSE5;
]
"vpmacswd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, AVXRegister, OWord]       ,  [0x08, 0x96        ], X, XOP_OP, SSE5 | AMD;
]
"vpmacsww" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, AVXRegister, OWord]       ,  [0x08, 0x95        ], X, XOP_OP, AMD | SSE5;
]
"vpmadcsswd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, AVXRegister, OWord]       ,  [0x08, 0xA6        ], X, XOP_OP, SSE5 | AMD;
]
"vpmadcswd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, AVXRegister, OWord]       ,  [0x08, 0xB6        ], X, XOP_OP, AMD | SSE5;
]
"vpmaddubsw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x04        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaddwd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xF5        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaskmovd" = [
    &[Memory, Auto, AVXRegister, Auto, AVXRegister, Auto]                            ,  [0x02, 0x8E        ], X, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX2;
    &[AVXRegister, Auto, AVXRegister, Auto, Memory, Auto]                            ,  [0x02, 0x8C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpmaskmovq" = [
    &[Memory, Auto, AVXRegister, Auto, AVXRegister, Auto]                            ,  [0x02, 0x8E        ], X, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX2;
    &[AVXRegister, Auto, AVXRegister, Auto, Memory, Auto]                            ,  [0x02, 0x8C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpmaxsb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x3C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaxsd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x3D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaxsw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xEE        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaxub" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xDE        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaxud" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x3F        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaxuw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x3E        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpminsb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x38        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpminsd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x39        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpminsw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xEA        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpminub" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xDA        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpminud" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x3B        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpminuw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x3A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovmskb" = [
    &[Legacy, Auto, AVXRegister, Auto]                                               ,  [0x01, 0xD7        ], X, VEX_OP | PREF_66, AVX;
]
"vpmovsxbd" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x02, 0x21        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, OWord]                                         ,  [0x02, 0x21        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovsxbq" = [
    &[AVXRegister, Auto, Memory, Word]                                               ,  [0x02, 0x22        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, OWord]                                         ,  [0x02, 0x22        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovsxbw" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x02, 0x20        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, Auto, Word, OWord]                                                ,  [0x02, 0x20        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovsxdq" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x02, 0x25        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, Auto, Word, OWord]                                                ,  [0x02, 0x25        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovsxwd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x02, 0x23        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, Auto, Word, OWord]                                                ,  [0x02, 0x23        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovsxwq" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x02, 0x24        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, OWord]                                         ,  [0x02, 0x24        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovzxbd" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x02, 0x31        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, OWord]                                         ,  [0x02, 0x31        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovzxbq" = [
    &[AVXRegister, Auto, Memory, Word]                                               ,  [0x02, 0x32        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, OWord]                                         ,  [0x02, 0x32        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovzxbw" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x02, 0x30        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, Auto, Word, OWord]                                                ,  [0x02, 0x30        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovzxdq" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x02, 0x35        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, Auto, Word, OWord]                                                ,  [0x02, 0x35        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovzxwd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x02, 0x33        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, Auto, Word, OWord]                                                ,  [0x02, 0x33        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovzxwq" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x02, 0x34        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, OWord]                                         ,  [0x02, 0x34        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmuldq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x28        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmulhrsw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x0B        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmulhuw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xE4        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmulhw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xE5        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmulld" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x40        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmullw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xD5        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmuludq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xF4        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpor" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xEB        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpperm" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord, AVXRegister, OWord]       ,  [0x08, 0xA3        ], X, XOP_OP, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Word, OWord]       ,  [0x08, 0xA3        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vprotb" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x08, 0xC0        ], X, XOP_OP, SSE5 | AMD;
    &[AVXRegister, OWord, Word, OWord, AVXRegister, OWord]                           ,  [0x09, 0x90        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x09, 0x90        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vprotd" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x08, 0xC2        ], X, XOP_OP, AMD | SSE5;
    &[AVXRegister, OWord, Word, OWord, AVXRegister, OWord]                           ,  [0x09, 0x92        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x09, 0x92        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vprotq" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x08, 0xC3        ], X, XOP_OP, AMD | SSE5;
    &[AVXRegister, OWord, Word, OWord, AVXRegister, OWord]                           ,  [0x09, 0x93        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x09, 0x93        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vprotw" = [
    &[AVXRegister, OWord, Word, OWord, Immediate, Byte]                              ,  [0x08, 0xC1        ], X, XOP_OP, AMD | SSE5;
    &[AVXRegister, OWord, Word, OWord, AVXRegister, OWord]                           ,  [0x09, 0x91        ], X, XOP_OP | ENC_MR, SSE5 | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x09, 0x91        ], X, WITH_REXW | XOP_OP, AMD | SSE5;
]
"vpsadbw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xF6        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpshab" = [
    &[AVXRegister, OWord, Word, OWord, AVXRegister, OWord]                           ,  [0x09, 0x98        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x09, 0x98        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vpshad" = [
    &[AVXRegister, OWord, Word, OWord, AVXRegister, OWord]                           ,  [0x09, 0x9A        ], X, XOP_OP | ENC_MR, SSE5 | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x09, 0x9A        ], X, WITH_REXW | XOP_OP, AMD | SSE5;
]
"vpshaq" = [
    &[AVXRegister, OWord, Word, OWord, AVXRegister, OWord]                           ,  [0x09, 0x9B        ], X, XOP_OP | ENC_MR, SSE5 | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x09, 0x9B        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vpshaw" = [
    &[AVXRegister, OWord, Word, OWord, AVXRegister, OWord]                           ,  [0x09, 0x99        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x09, 0x99        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vpshlb" = [
    &[AVXRegister, OWord, Word, OWord, AVXRegister, OWord]                           ,  [0x09, 0x94        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x09, 0x94        ], X, WITH_REXW | XOP_OP, AMD | SSE5;
]
"vpshld" = [
    &[AVXRegister, OWord, Word, OWord, AVXRegister, OWord]                           ,  [0x09, 0x96        ], X, XOP_OP | ENC_MR, SSE5 | AMD;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x09, 0x96        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vpshlq" = [
    &[AVXRegister, OWord, Word, OWord, AVXRegister, OWord]                           ,  [0x09, 0x97        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x09, 0x97        ], X, WITH_REXW | XOP_OP, AMD | SSE5;
]
"vpshlw" = [
    &[AVXRegister, OWord, Word, OWord, AVXRegister, OWord]                           ,  [0x09, 0x95        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    &[AVXRegister, OWord, AVXRegister, OWord, Word, OWord]                           ,  [0x09, 0x95        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vpshufb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x00        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpshufd" = [
    &[AVXRegister, Auto, Word, Auto, Immediate, Byte]                                ,  [0x01, 0x70        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpshufhw" = [
    &[AVXRegister, Auto, Word, Auto, Immediate, Byte]                                ,  [0x01, 0x70        ], X, VEX_OP | AUTO_VEXL | PREF_F3, AVX;
]
"vpshuflw" = [
    &[AVXRegister, Auto, Word, Auto, Immediate, Byte]                                ,  [0x01, 0x70        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
]
"vpsignb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x08        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsignd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x0A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsignw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x09        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpslld" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Immediate, Byte]                         ,  [0x01, 0x72        ], 6, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, OWord]                             ,  [0x01, 0xF2        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpslldq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Immediate, Byte]                         ,  [0x01, 0x73        ], 7, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
]
"vpsllq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Immediate, Byte]                         ,  [0x01, 0x73        ], 6, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, OWord]                             ,  [0x01, 0xF3        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsllvd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x47        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpsllvq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x47        ], X, VEX_OP | AUTO_VEXL | WITH_REXW | PREF_66, AVX2;
]
"vpsllw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Immediate, Byte]                         ,  [0x01, 0x71        ], 6, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, OWord]                             ,  [0x01, 0xF1        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsrad" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Immediate, Byte]                         ,  [0x01, 0x72        ], 4, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, OWord]                             ,  [0x01, 0xE2        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsravd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x46        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpsraw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Immediate, Byte]                         ,  [0x01, 0x71        ], 4, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, OWord]                             ,  [0x01, 0xE1        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsrld" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Immediate, Byte]                         ,  [0x01, 0x72        ], 2, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, OWord]                             ,  [0x01, 0xD2        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsrldq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Immediate, Byte]                         ,  [0x01, 0x73        ], 3, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
]
"vpsrlq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Immediate, Byte]                         ,  [0x01, 0x73        ], 2, VEX_OP | ENC_VM | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, OWord]                             ,  [0x01, 0xD3        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsrlvd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x45        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpsrlvq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x02, 0x45        ], X, VEX_OP | AUTO_VEXL | WITH_REXW | PREF_66, AVX2;
]
"vpsrlw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Immediate, Byte]                         ,  [0x01, 0x71        ], 2, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    &[AVXRegister, Auto, AVXRegister, Auto, Word, OWord]                             ,  [0x01, 0xD1        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xF8        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xFA        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xFB        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubsb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xE8        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubsw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xE9        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubusb" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xD8        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubusw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xD9        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xF9        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vptest" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x02, 0x17        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpckhbw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x68        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpckhdq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x6A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpckhqdq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x6D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpckhwd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x69        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpcklbw" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x60        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpckldq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x62        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpcklqdq" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x6C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpcklwd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x61        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpxor" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0xEF        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vrcpps" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x53        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vrcpss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x01, 0x53        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x53        ], X, VEX_OP | PREF_F3, AVX;
]
"vroundpd" = [
    &[AVXRegister, Auto, Word, Auto, Immediate, Byte]                                ,  [0x03, 0x09        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vroundps" = [
    &[AVXRegister, Auto, Word, Auto, Immediate, Byte]                                ,  [0x03, 0x08        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vroundsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord, Immediate, Byte]        ,  [0x03, 0x0B        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]   ,  [0x03, 0x0B        ], X, VEX_OP | PREF_66, AVX;
]
"vroundss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord, Immediate, Byte]        ,  [0x03, 0x0A        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord, Immediate, Byte]   ,  [0x03, 0x0A        ], X, VEX_OP | PREF_66, AVX;
]
"vrsqrtps" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x52        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vrsqrtss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x01, 0x52        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x52        ], X, VEX_OP | PREF_F3, AVX;
]
"vshufpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, Immediate, Byte]             ,  [0x01, 0xC6        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vshufps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto, Immediate, Byte]             ,  [0x01, 0xC6        ], X, VEX_OP | AUTO_VEXL | ENC_MR, AVX;
]
"vsqrtpd" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x51        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vsqrtps" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x01, 0x51        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vsqrtsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0x51        ], X, VEX_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x51        ], X, VEX_OP | PREF_F2, AVX;
]
"vsqrtss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x01, 0x51        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x51        ], X, VEX_OP | PREF_F3, AVX;
]
"vstmxcsr" = [
    &[Memory, DWord]                                                                 ,  [0x01, 0xAE        ], 3, VEX_OP, AVX;
]
"vsubpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x5C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vsubps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x5C        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vsubsd" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, QWord]                         ,  [0x01, 0x5C        ], X, VEX_OP | PREF_F2, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x5C        ], X, VEX_OP | PREF_F2, AVX;
]
"vsubss" = [
    &[AVXRegister, OWord, AVXRegister, OWord, Memory, DWord]                         ,  [0x01, 0x5C        ], X, VEX_OP | PREF_F3, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord, AVXRegister, OWord]                    ,  [0x01, 0x5C        ], X, VEX_OP | PREF_F3, AVX;
]
"vtestpd" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x02, 0x0F        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vtestps" = [
    &[AVXRegister, Auto, Word, Auto]                                                 ,  [0x02, 0x0E        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vucomisd" = [
    &[AVXRegister, OWord, Memory, QWord]                                             ,  [0x01, 0x2E        ], X, VEX_OP | PREF_66, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x01, 0x2E        ], X, VEX_OP | PREF_66, AVX;
]
"vucomiss" = [
    &[AVXRegister, OWord, Memory, DWord]                                             ,  [0x01, 0x2E        ], X, VEX_OP, AVX;
    &[AVXRegister, OWord, AVXRegister, OWord]                                        ,  [0x01, 0x2E        ], X, VEX_OP, AVX;
]
"vunpckhpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x15        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vunpckhps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x15        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vunpcklpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x14        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vunpcklps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x14        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vxorpd" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x57        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vxorps" = [
    &[AVXRegister, Auto, AVXRegister, Auto, Word, Auto]                              ,  [0x01, 0x57        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vzeroall" = [
    &[]                                                                              ,  [0x01, 0x77        ], X, WITH_VEXL | VEX_OP, AVX;
]
"vzeroupper" = [
    &[]                                                                              ,  [0x01, 0x77        ], X, VEX_OP, AVX;
]
"wbinvd" = [
    &[]                                                                              ,  [0x0F, 0x09        ], X;
]
"wrfsbase" = [
    &[Legacy, DWord]                                                                 ,  [0x0F, 0xAE        ], 2, PREF_F3;
    &[Legacy, QWord]                                                                 ,  [0x0F, 0xAE        ], 2, WITH_REXW | PREF_F3;
]
"wrgsbase" = [
    &[Legacy, DWord]                                                                 ,  [0x0F, 0xAE        ], 3, PREF_F3;
    &[Legacy, QWord]                                                                 ,  [0x0F, 0xAE        ], 3, WITH_REXW | PREF_F3;
]
"wrmsr" = [
    &[]                                                                              ,  [0x0F, 0x30        ], X;
]
"wrpkru" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xEF  ], X;
]
"wrshr" = [
    &[LegacyMemory, DWord]                                                           ,  [0x0F, 0x37        ], 0, DEFAULT, CYRIX;
]
"xabort" = [
    &[Immediate, Byte]                                                               ,  [0xC6, 0xF8        ], X, DEFAULT, RTM;
]
"xadd" = [
    &[Memory, Byte, Legacy, Byte]                                                    ,  [0x0F, 0xC0        ], X, LOCK | ENC_MR;
    &[Legacy, Byte, Legacy, Byte]                                                    ,  [0x0F, 0xC0        ], X, ENC_MR;
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x0F, 0xC1        ], X, AUTO_SIZE | LOCK | ENC_MR;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x0F, 0xC1        ], X, AUTO_SIZE | ENC_MR;
]
"xbegin" = [
    &[OWord, DWord]                                                                  ,  [0xC7, 0xF8        ], X, DEFAULT, RTM;
]
"xchg" = [
    &[Memory, Byte, Legacy, Byte]                                                    ,  [0x86              ], X, LOCK | ENC_MR;
    &[Legacy, Byte, Memory, Byte]                                                    ,  [0x86              ], X, LOCK;
    &[Legacy, Byte, Legacy, Byte]                                                    ,  [0x86              ], X;
    &[Legacy, Byte, Legacy, Byte]                                                    ,  [0x86              ], X, ENC_MR;
    &[rax, Auto, Legacy, Auto]                                                       ,  [0x90              ], X, AUTO_SIZE | SHORT_ARG;
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x87              ], X, AUTO_SIZE | ENC_MR;
    &[Legacy, Auto, rax, Auto]                                                       ,  [0x90              ], X, AUTO_SIZE | SHORT_ARG;
    &[Legacy, Auto, Memory, Auto]                                                    ,  [0x87              ], X, AUTO_SIZE;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x87              ], X, AUTO_SIZE;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x87              ], X, AUTO_SIZE | ENC_MR;
]
"xcryptcbc" = [
    &[]                                                                              ,  [0x0F, 0xA7, 0xD0  ], X, PREF_F3, CYRIX;
]
"xcryptcfb" = [
    &[]                                                                              ,  [0x0F, 0xA7, 0xE0  ], X, PREF_F3, CYRIX;
]
"xcryptctr" = [
    &[]                                                                              ,  [0x0F, 0xA7, 0xD8  ], X, PREF_F3, CYRIX;
]
"xcryptecb" = [
    &[]                                                                              ,  [0x0F, 0xA7, 0xC8  ], X, PREF_F3, CYRIX;
]
"xcryptofb" = [
    &[]                                                                              ,  [0x0F, 0xA7, 0xE8  ], X, PREF_F3, CYRIX;
]
"xend" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xD5  ], X, DEFAULT, RTM;
]
"xgetbv" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xD0  ], X;
]
"xlat" = [
    &[]                                                                              ,  [0xD7              ], X;
]
"xlatb" = [
    &[]                                                                              ,  [0xD7              ], X;
]
"xor" = [
    &[rax, Byte, Immediate, Byte]                                                    ,  [0x34              ], X;
    &[Memory, Byte, Immediate, Byte]                                                 ,  [0x80              ], 6, LOCK;
    &[Memory, Byte, Legacy, Byte]                                                    ,  [0x30              ], X, LOCK | ENC_MR;
    &[Legacy, Byte, Immediate, Byte]                                                 ,  [0x80              ], 6;
    &[Legacy, Byte, Legacy, Byte]                                                    ,  [0x30              ], X, ENC_MR;
    &[Legacy, Byte, LegacyMemory, Byte]                                              ,  [0x32              ], X;
    &[Legacy, Auto, Immediate, Byte]                                                 ,  [0x83              ], 6, AUTO_SIZE  | EXACT_SIZE;
    &[rax, Auto, Immediate, Auto]                                                    ,  [0x35              ], X, AUTO_SIZE;
    &[Memory, Auto, Immediate, Auto]                                                 ,  [0x81              ], 6, AUTO_SIZE | LOCK;
    &[Memory, Auto, Immediate, Byte]                                                 ,  [0x83              ], 6, AUTO_SIZE | LOCK;
    &[Memory, Auto, Legacy, Auto]                                                    ,  [0x31              ], X, AUTO_SIZE | LOCK | ENC_MR;
    &[Legacy, Auto, Immediate, Auto]                                                 ,  [0x81              ], 6, AUTO_SIZE ;
    &[Legacy, Auto, Legacy, Auto]                                                    ,  [0x31              ], X, AUTO_SIZE | ENC_MR;
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x33              ], X, AUTO_SIZE;
]
"xorpd" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x57        ], X, PREF_66, SSE2;
]
"xorps" = [
    &[AVXRegister, OWord, Word, OWord]                                               ,  [0x0F, 0x57        ], X, DEFAULT, SSE;
]
"xrstor" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xAE        ], 5;
]
"xrstor64" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xAE        ], 5, WITH_REXW;
]
"xrstors64" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xC7        ], 3, WITH_REXW;
]
"xsave" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xAE        ], 4;
]
"xsave64" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xAE        ], 4, WITH_REXW;
]
"xsavec64" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xC7        ], 4, WITH_REXW;
]
"xsaveopt64" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xAE        ], 6, WITH_REXW;
]
"xsaves64" = [
    &[Memory, MemAuto]                                                               ,  [0x0F, 0xC7        ], 5, WITH_REXW;
]
"xsetbv" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xD1  ], X;
]
"xsha1" = [
    &[]                                                                              ,  [0x0F, 0xA6, 0xC8  ], X, PREF_F3, CYRIX;
]
"xsha256" = [
    &[]                                                                              ,  [0x0F, 0xA6, 0xD0  ], X, PREF_F3, CYRIX;
]
"xstore" = [
    &[]                                                                              ,  [0x0F, 0xA7, 0xC0  ], X, DEFAULT, CYRIX;
]
"xtest" = [
    &[]                                                                              ,  [0x0F, 0x01, 0xD6  ], X, DEFAULT, RTM;
]

"call"  = [
    &[Immediate, Word, Immediate, Word]                                              ,  [0x9A              ], X, X86_ONLY | WORD_SIZE | EXACT_SIZE;
    &[Immediate, DWord, Immediate, Word]                                             ,  [0x9A              ], X, X86_ONLY;
    &[Memory, FWord]                                                                 ,  [0xFF              ], 3, X86_ONLY | EXACT_SIZE;
    &[OWord, DWord]                                                                  ,  [0xE8              ], X;
    &[LegacyMemory, Auto]                                                            ,  [0xFF              ], 2, AUTO_NO32;
]
"callf" = [
    &[Immediate, Word, Immediate, Word]                                              ,  [0x9A              ], X, X86_ONLY | WORD_SIZE | EXACT_SIZE;
    &[Immediate, DWord, Immediate, Word]                                             ,  [0x9A              ], X, X86_ONLY;
    &[Memory, DWord]                                                                 ,  [0xFF              ], 3, X86_ONLY | WORD_SIZE | EXACT_SIZE;
    &[Memory, FWord]                                                                 ,  [0xFF              ], 3, X86_ONLY;
]
"jmp"   = [
    &[Immediate, Word, Immediate, Word]                                              ,  [0x9A              ], X, X86_ONLY | WORD_SIZE | EXACT_SIZE;
    &[Immediate, DWord, Immediate, Word]                                             ,  [0xEA              ], X, X86_ONLY;
    &[Memory, FWord]                                                                 ,  [0xFF              ], 5, X86_ONLY | EXACT_SIZE;
    &[OWord, Byte]                                                                   ,  [0xEB              ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,  [0xE9              ], X;
    &[LegacyMemory, Auto]                                                            ,  [0xFF              ], 4, AUTO_NO32 ;
]
"jmpf" = [
    &[Immediate, Word, Immediate, Word]                                              ,  [0x9A              ], X, X86_ONLY | WORD_SIZE | EXACT_SIZE;
    &[Immediate, DWord, Immediate, Word]                                             ,  [0xEA              ], X, X86_ONLY;
    &[Memory, DWord]                                                                 ,  [0xFF              ], 5, X86_ONLY | WORD_SIZE | EXACT_SIZE;
    &[Memory, FWord]                                                                 ,  [0xFF              ], 5, X86_ONLY;
]
"mov"   = [
    &[LegacyMemory, Auto, Legacy, Auto]                                              ,  [0x89              ], X, AUTO_SIZE;
    &[LegacyMemory, Byte, Legacy, Byte]                                              ,  [0x88              ], X;
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x8B              ], X, AUTO_SIZE;
    &[Legacy, Byte, LegacyMemory, Byte]                                              ,  [0x8A              ], X;
    &[Legacy, Auto, Segment, Word]                                                   ,  [0x8C              ], X, AUTO_SIZE;
    &[Memory, Word, Segment, Word]                                                   ,  [0x8C              ], X;
    &[Segment, Word, Memory, Word]                                                   ,  [0x8C              ], X;
    &[Segment, Word, Legacy, Word]                                                   ,  [0x8C              ], X;
    &[Legacy, Byte, Immediate, Byte]                                                 ,  [0xB0              ], X,             SHORT_ARG;
    &[Legacy, Word, Immediate, Word]                                                 ,  [0xB8              ], X, WORD_SIZE | SHORT_ARG;
    &[Legacy, DWord, Immediate, DWord]                                               ,  [0xB8              ], X,             SHORT_ARG;
    &[LegacyMemory, Auto, Immediate, Auto]                                           ,  [0xC7              ], 0, AUTO_SIZE;
    &[LegacyMemory, Byte, Immediate, Byte]                                           ,  [0xC6              ], 0;
    &[Legacy, QWord, Immediate, QWord]                                               ,  [0xB8              ], X, WITH_REXW | SHORT_ARG;
    &[Control, DWord, Legacy, DWord]                                                 ,  [0x0F, 0x22        ], X; // can only match in 32 bit mode due to "cd"
    &[Control, QWord, Legacy, QWord]                                                 ,  [0x0F, 0x22        ], X; // doesn't need a prefix to be encoded, as it's 64 bit natural in 64 bit mode
    &[Legacy, DWord, Control, DWord]                                                 ,  [0x0F, 0x20        ], X;
    &[Legacy, QWord, Control, QWord]                                                 ,  [0x0F, 0x20        ], X;
    &[CR8, DWord, Legacy, DWord]                                                     ,  [0x0F, 0x22        ], 0, PREF_F0; // note: technically CR8 should actually be encoded, but the encoding is 0.
    &[CR8, QWord, Legacy, QWord]                                                     ,  [0x0F, 0x22        ], 0, PREF_F0;
    &[Legacy, DWord, CR8, DWord]                                                     ,  [0x0F, 0x22        ], 0, PREF_F0;
    &[Legacy, QWord, CR8, QWord]                                                     ,  [0x0F, 0x22        ], 0, PREF_F0;
    &[DWord, DWord, Legacy, DWord]                                                   ,  [0x0F, 0x23        ], X; // 32 bit mode only
    &[DWord, QWord, Legacy, QWord]                                                   ,  [0x0F, 0x23        ], X;
    &[Legacy, DWord, DWord, DWord]                                                   ,  [0x0F, 0x21        ], X;
    &[Legacy, QWord, DWord, QWord]                                                   ,  [0x0F, 0x21        ], X;
]
"movabs"  = [
    &[rax, Byte, Immediate, QWord]                                                   ,  [0xA0              ], X; // special syntax for 64-bit disp only mov
    &[rax, Word, Immediate, QWord]                                                   ,  [0xA1              ], X, WORD_SIZE;
    &[rax, DWord, Immediate, QWord]                                                  ,  [0xA1              ], X;
    &[rax, QWord, Immediate, QWord]                                                  ,  [0xA1              ], X, WITH_REXW;
    &[Immediate, QWord, rax, Byte]                                                   ,  [0xA2              ], X;
    &[Immediate, QWord, rax, Word]                                                   ,  [0xA3              ], X, WORD_SIZE;
    &[Immediate, QWord, rax, DWord]                                                  ,  [0xA3              ], X;
    &[Immediate, QWord, rax, QWord]                                                  ,  [0xA3              ], X, WITH_REXW;
]
"jo"     = [
    &[OWord, Byte]                                                                   ,  [0x70            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,  [0x0F, 0x80      ], X;
]
"jno"    = [
    &[OWord, Byte]                                                                   ,  [0x71            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,  [0x0F, 0x81      ], X;
]
"jb"     = [
    &[OWord, Byte]                                                                   ,        [0x72            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x82      ], X;
]
"jc"     = [
    &[OWord, Byte]                                                                   ,        [0x72            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x82      ], X;
]
"jnae"   = [
    &[OWord, Byte]                                                                   ,        [0x72            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x82      ], X;
]
"jnb"    = [
    &[OWord, Byte]                                                                   ,        [0x73            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x83      ], X;
]
"jnc"    = [
    &[OWord, Byte]                                                                   ,        [0x73            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x83      ], X;
]
"jae"    = [
    &[OWord, Byte]                                                                   ,        [0x73            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x83      ], X;
]
"jz"     = [
    &[OWord, Byte]                                                                   ,        [0x74            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x84      ], X;
]
"je"     = [
    &[OWord, Byte]                                                                   ,        [0x74            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x84      ], X;
]
"jnz"    = [
    &[OWord, Byte]                                                                   ,        [0x75            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x85      ], X;
]
"jne"    = [
    &[OWord, Byte]                                                                   ,        [0x75            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x85      ], X;
]
"jbe"    = [
    &[OWord, Byte]                                                                   ,        [0x76            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x86      ], X;
]
"jna"    = [
    &[OWord, Byte]                                                                   ,        [0x76            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x86      ], X;
]
"jnbe"   = [
    &[OWord, Byte]                                                                   ,        [0x77            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x87      ], X;
]
"ja"     = [
    &[OWord, Byte]                                                                   ,        [0x77            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x87      ], X;
]
"js"     = [
    &[OWord, Byte]                                                                   ,        [0x78            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x88      ], X;
]
"jns"    = [
    &[OWord, Byte]                                                                   ,        [0x79            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x89      ], X;
]
"jp"     = [
    &[OWord, Byte]                                                                   ,        [0x7A            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x8A      ], X;
]
"jpe"    = [
    &[OWord, Byte]                                                                   ,        [0x7A            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x8A      ], X;
]
"jnp"    = [
    &[OWord, Byte]                                                                   ,        [0x7B            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x8B      ], X;
]
"jpo"    = [
    &[OWord, Byte]                                                                   ,        [0x7B            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x8B      ], X;
]
"jl"     = [
    &[OWord, Byte]                                                                   ,        [0x7C            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x8C      ], X;
]
"jnge"   = [
    &[OWord, Byte]                                                                   ,        [0x7C            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x8C      ], X;
]
"jnl"    = [
    &[OWord, Byte]                                                                   ,        [0x7D            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x8D      ], X;
]
"jge"    = [
    &[OWord, Byte]                                                                   ,        [0x7D            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x8D      ], X;
]
"jle"    = [
    &[OWord, Byte]                                                                   ,        [0x7E            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x8E      ], X;
]
"jng"    = [
    &[OWord, Byte]                                                                   ,        [0x7E            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x8E      ], X;
]
"jnle"   = [
    &[OWord, Byte]                                                                   ,        [0x7F            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x8F      ], X;
]
"jg"     = [
    &[OWord, Byte]                                                                   ,        [0x7F            ], X, EXACT_SIZE;
    &[OWord, DWord]                                                                  ,        [0x0F, 0x8F      ], X;
]

"cmovo"    = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x40      ], X, AUTO_SIZE;
]
"cmovno"   = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x41      ], X, AUTO_SIZE;
]
"cmovb"    = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x42      ], X, AUTO_SIZE;
]
"cmovc"    = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x42      ], X, AUTO_SIZE;
]
"cmovnae"  = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x42      ], X, AUTO_SIZE;
]
"cmovnb"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x43      ], X, AUTO_SIZE;
]
"cmovnc"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x43      ], X, AUTO_SIZE;
]
"cmovae"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x43      ], X, AUTO_SIZE;
]
"cmovz"       = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x44      ], X, AUTO_SIZE;
]
"cmove"       = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x44      ], X, AUTO_SIZE;
]
"cmovnz"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x45      ], X, AUTO_SIZE;
]
"cmovne"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x45      ], X, AUTO_SIZE;
]
"cmovbe"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x46      ], X, AUTO_SIZE;
]
"cmovna"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x46      ], X, AUTO_SIZE;
]
"cmovnbe"     = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x47      ], X, AUTO_SIZE;
]
"cmova"       = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x47      ], X, AUTO_SIZE;
]
"cmovs"       = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x48      ], X, AUTO_SIZE;
]
"cmovns"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x49      ], X, AUTO_SIZE;
]
"cmovp"       = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x4A      ], X, AUTO_SIZE;
]
"cmovpe"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x4A      ], X, AUTO_SIZE;
]
"cmovnp"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x4B      ], X, AUTO_SIZE;
]
"cmovpo"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x4B      ], X, AUTO_SIZE;
]
"cmovl"       = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x4C      ], X, AUTO_SIZE;
]
"cmovnge"     = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x4C      ], X, AUTO_SIZE;
]
"cmovnl"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x4D      ], X, AUTO_SIZE;
]
"cmovge"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x4D      ], X, AUTO_SIZE;
]
"cmovle"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x4E      ], X, AUTO_SIZE;
]
"cmovng"      = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x4E      ], X, AUTO_SIZE;
]
"cmovnle"     = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x4F      ], X, AUTO_SIZE;
]
"cmovg"       = [
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,      [0x0F, 0x4F      ], X, AUTO_SIZE;
]

"seto"        = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x90      ], 0;
]
"setno"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x91      ], 0;
]
"setb"        = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x92      ], 0;
]
"setc"        = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x92      ], 0;
]
"setnae"      = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x92      ], 0;
]
"setnb"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x93      ], 0;
]
"setnc"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x93      ], 0;
]
"setae"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x93      ], 0;
]
"setz"        = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x94      ], 0;
]
"sete"        = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x94      ], 0;
]
"setnz"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x95      ], 0;
]
"setne"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x95      ], 0;
]
"setbe"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x96      ], 0;
]
"setna"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x96      ], 0;
]
"setnbe"      = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x97      ], 0;
]
"seta"        = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x97      ], 0;
]
"sets"        = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x98      ], 0;
]
"setns"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x99      ], 0;
]
"setp"        = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x9A      ], 0;
]
"setpe"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x9A      ], 0;
]
"setnp"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x9B      ], 0;
]
"setpo"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x9B      ], 0;
]
"setl"        = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x9C      ], 0;
]
"setnge"      = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x9C      ], 0;
]
"setnl"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x9D      ], 0;
]
"setge"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x9D      ], 0;
]
"setle"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x9E      ], 0;
]
"setng"       = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x9E      ], 0;
]
"setnle"      = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x9F      ], 0;
]
"setg"        = [
    &[LegacyMemory, Byte]                                                            ,        [0x0F, 0x9F      ], 0;
]


"in"    = [
    &[rax, Byte, Immediate, Byte]                                                    ,  [0xE4            ], X;
    &[rax, Word, Immediate, Byte]                                                    ,  [0xE5            ], X, WORD_SIZE;
    &[rax, DWord, Immediate, Byte]                                                   ,  [0xE5            ], X;
    &[rax, Byte, rdx, Word]                                                          ,  [0xEC            ], X;
    &[rax, Word, rdx, Word]                                                          ,  [0xED            ], X, WORD_SIZE;
    &[rax, DWord, rdx, Word]                                                         ,  [0xED            ], X;
]

"out"   = [
    &[Immediate, Byte, rax, Byte]                                                    ,  [0xE6            ], X;
    &[Immediate, Byte, rax, Word]                                                    ,  [0xE7            ], X;
    &[Immediate, Byte, rax, DWord]                                                   ,  [0xE7            ], X;
    &[rdx, Word, rax, Byte]                                                          ,  [0xEE            ], X;
    &[rdx, Word, rax, Word]                                                          ,  [0xEF            ], X, WORD_SIZE;
    &[rdx, Word, rax, DWord]                                                         ,  [0xEF            ], X;
]

"crc32"  = [
    &[Legacy, Auto, LegacyMemory, Byte]                                              ,  [0x0F, 0x38, 0xF0], X, AUTO_REXW | PREF_F2 | EXACT_SIZE; // unique size encoding scheme
    &[Legacy, DWord, LegacyMemory, Word]                                             ,  [0x0F, 0x38, 0xF1], X, WORD_SIZE | PREF_F2 | EXACT_SIZE;
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x0F, 0x38, 0xF1], X, AUTO_REXW | PREF_F2 | EXACT_SIZE;
]

"imul"   = [
    &[LegacyMemory, Auto]                                                            ,  [0xF7            ], 5, AUTO_SIZE;
    &[LegacyMemory, Byte]                                                            ,  [0xF6            ], 5;
    &[Legacy, Auto, LegacyMemory, Auto]                                              ,  [0x0F, 0xAF      ], X, AUTO_SIZE;
    &[Legacy, Auto, LegacyMemory, Auto, Immediate, Byte]                             ,  [0x6B            ], X, AUTO_SIZE | EXACT_SIZE;
    &[Legacy, Auto, LegacyMemory, Auto, Immediate, Auto]                             ,  [0x69            ], X, AUTO_SIZE;
]

)
