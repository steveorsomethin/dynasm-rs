Ops!(

"aaa" = [
    []                                                                                                                                          ,  [0x37              ], X, X86_ONLY;
]
"aad" = [
    []                                                                                                                                          ,  [0xD5, 0x0A        ], X, X86_ONLY;
]
"aam" = [
    []                                                                                                                                          ,  [0xD4, 0x0A        ], X, X86_ONLY;
]
"aas" = [
    []                                                                                                                                          ,  [0x3F              ], X, X86_ONLY;
]
"adc" = [
    [RegisterX64::rax, Size::Byte, OperandType::Immediate, Size::Byte]                                                                          ,  [0x14              ], X;
    [OperandType::Memory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                       ,  [0x80              ], 2, LOCK;
    [OperandType::Memory, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                         ,  [0x10              ], X, LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::Immediate, Size::Byte]                                                                      ,  [0x80              ], 2;
    [RegisterType::Legacy, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                        ,  [0x10              ], X, ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::LegacyMemory, Size::Byte]                                                                   ,  [0x12              ], X;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Byte]                                                                      ,  [0x83              ], 2, AUTO_SIZE  | EXACT_SIZE;
    [RegisterX64::rax, Size::Auto, OperandType::Immediate, Size::Auto]                                                                          ,  [0x15              ], X, AUTO_SIZE;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Auto]                                                                       ,  [0x81              ], 2, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                       ,  [0x83              ], 2, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x11              ], X, AUTO_SIZE | LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Auto]                                                                      ,  [0x81              ], 2, AUTO_SIZE ;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x11              ], X, AUTO_SIZE | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x13              ], X, AUTO_SIZE;
]
"adcx" = [
    [RegisterType::Legacy, Size::QWord, OperandType::LegacyMemory, Size::QWord]                                                                 ,  [0x0F, 0x38, 0xF6  ], X, WITH_REXW | PREF_66;
]
"add" = [
    [RegisterX64::rax, Size::Byte, OperandType::Immediate, Size::Byte]                                                                          ,  [0x04              ], X;
    [OperandType::Memory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                       ,  [0x80              ], 0, LOCK;
    [OperandType::Memory, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                         ,  [0x00              ], X, LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::Immediate, Size::Byte]                                                                      ,  [0x80              ], 0;
    [RegisterType::Legacy, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                        ,  [0x00              ], X, ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::LegacyMemory, Size::Byte]                                                                   ,  [0x02              ], X;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Byte]                                                                      ,  [0x83              ], 0, AUTO_SIZE  | EXACT_SIZE;
    [RegisterX64::rax, Size::Auto, OperandType::Immediate, Size::Auto]                                                                          ,  [0x05              ], X, AUTO_SIZE;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Auto]                                                                       ,  [0x81              ], 0, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                       ,  [0x83              ], 0, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x01              ], X, AUTO_SIZE | LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Auto]                                                                      ,  [0x81              ], 0, AUTO_SIZE ;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x01              ], X, AUTO_SIZE | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x03              ], X, AUTO_SIZE;
]
"addpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x58        ], X, PREF_66, SSE2;
]
"addps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x58        ], X, DEFAULT, SSE;
]
"addsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x58        ], X, PREF_F2, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x58        ], X, PREF_F2, SSE2;
]
"addss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x58        ], X, PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x58        ], X, PREF_F3, SSE;
]
"addsubpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xD0        ], X, PREF_66, SSE3;
]
"addsubps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xD0        ], X, PREF_F2, SSE3;
]
"adox" = [
    [RegisterType::Legacy, Size::QWord, OperandType::LegacyMemory, Size::QWord]                                                                 ,  [0x0F, 0x38, 0xF6  ], X, WITH_REXW | PREF_F3;
]
"aesdec" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x38, 0xDE  ], X, PREF_66, SSE;
]
"aesdeclast" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x38, 0xDF  ], X, PREF_66, SSE;
]
"aesenc" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x38, 0xDC  ], X, PREF_66, SSE;
]
"aesenclast" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x38, 0xDD  ], X, PREF_66, SSE;
]
"aesimc" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x38, 0xDB  ], X, PREF_66, SSE;
]
"aeskeygenassist" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x0F, 0x3A, 0xDF  ], X, PREF_66, SSE;
]
"and" = [
    [RegisterX64::rax, Size::Byte, OperandType::Immediate, Size::Byte]                                                                          ,  [0x24              ], X;
    [OperandType::Memory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                       ,  [0x80              ], 4, LOCK;
    [OperandType::Memory, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                         ,  [0x20              ], X, LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::Immediate, Size::Byte]                                                                      ,  [0x80              ], 4;
    [RegisterType::Legacy, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                        ,  [0x20              ], X, ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::LegacyMemory, Size::Byte]                                                                   ,  [0x22              ], X;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Byte]                                                                      ,  [0x83              ], 4, AUTO_SIZE  | EXACT_SIZE;
    [RegisterX64::rax, Size::Auto, OperandType::Immediate, Size::Auto]                                                                          ,  [0x25              ], X, AUTO_SIZE;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Auto]                                                                       ,  [0x81              ], 4, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                       ,  [0x83              ], 4, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x21              ], X, AUTO_SIZE | LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Auto]                                                                      ,  [0x81              ], 4, AUTO_SIZE ;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x21              ], X, AUTO_SIZE | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x23              ], X, AUTO_SIZE;
]
"andn" = [
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                 ,  [0x02, 0xF2        ], X, VEX_OP | AUTO_REXW, BMI1;
]
"andnpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x55        ], X, PREF_66, SSE2;
]
"andnps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x55        ], X, DEFAULT, SSE;
]
"andpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x54        ], X, PREF_66, SSE2;
]
"andps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x54        ], X, DEFAULT, SSE;
]
"arpl" = [
    [OperandType::LegacyMemory, Size::Word, RegisterType::Legacy, Size::Word]                                                                   ,  [0x63              ], X, X86_ONLY;
]
"bextr" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::DWord]                              ,  [0x10, 0x10        ], X, XOP_OP | AUTO_REXW, TBM;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto, RegisterType::Legacy, Size::Auto]                                 ,  [0x02, 0xF7        ], X, VEX_OP | AUTO_REXW | ENC_MR, BMI1;
]
"blcfill" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x09, 0x01        ], 1, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blci" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x09, 0x02        ], 6, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blcic" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x09, 0x01        ], 5, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blcmsk" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x09, 0x02        ], 1, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blcs" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x09, 0x01        ], 3, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blendpd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x0D  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x0D  ], X, PREF_66, SSE41;
]
"blendps" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x0C  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x0C  ], X, PREF_66, SSE41;
]
"blendvpd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x15  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x15  ], X, PREF_66, SSE41;
]
"blendvps" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x14  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x14  ], X, PREF_66, SSE41;
]
"blsfill" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x09, 0x01        ], 2, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blsi" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x02, 0xF3        ], 3, VEX_OP | AUTO_REXW | ENC_VM, BMI1;
]
"blsic" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x09, 0x01        ], 6, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"blsmsk" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x02, 0xF3        ], 2, VEX_OP | AUTO_REXW | ENC_VM, BMI1;
]
"blsr" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x02, 0xF3        ], 1, VEX_OP | AUTO_REXW | ENC_VM, BMI1;
]
"bound" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::MemAuto]                                                                      ,  [0x62              ], X, AUTO_SIZE | X86_ONLY;
]
"bndcl" = [
    [Size::Byte, Size::OWord, OperandType::Memory, Size::MemAuto]                                                                               ,  [0x0F, 0x1A        ], X, PREF_F3, MPX;
    [Size::Byte, Size::OWord, RegisterType::Legacy, Size::QWord]                                                                                ,  [0x0F, 0x1A        ], X,  PREF_F3, MPX;
]
"bndcn" = [
    [Size::Byte, Size::OWord, OperandType::Memory, Size::MemAuto]                                                                               ,  [0x0F, 0x1B        ], X, PREF_F2, MPX;
    [Size::Byte, Size::OWord, RegisterType::Legacy, Size::QWord]                                                                                ,  [0x0F, 0x1B        ], X,  PREF_F2, MPX;
]
"bndcu" = [
    [Size::Byte, Size::OWord, OperandType::Memory, Size::MemAuto]                                                                               ,  [0x0F, 0x1A        ], X, PREF_F2, MPX;
    [Size::Byte, Size::OWord, RegisterType::Legacy, Size::QWord]                                                                                ,  [0x0F, 0x1A        ], X,  PREF_F2, MPX;
]
"bndldx" = [
    [Size::Byte, Size::OWord, OperandType::Memory, Size::MemAuto]                                                                               ,  [0x0F, 0x1A        ], X, ENC_MIB, MPX;
]
"bndmk" = [
    [Size::Byte, Size::OWord, OperandType::Memory, Size::MemAuto]                                                                               ,  [0x0F, 0x1B        ], X, ENC_MIB | PREF_F3, MPX;
]
"bndmov" = [
    [Size::Byte, Size::OWord, Size::Byte, Size::OWord]                                                                                          ,  [0x0F, 0x1A        ], X, PREF_66, MPX;
    [Size::Byte, Size::OWord, Size::Byte, Size::OWord]                                                                                          ,  [0x0F, 0x1B        ], X, ENC_MR | PREF_66, MPX;
    [Size::Byte, Size::OWord, OperandType::Memory, Size::MemAuto]                                                                               ,  [0x0F, 0x1A        ], X, PREF_66, MPX;
    [OperandType::Memory, Size::MemAuto, Size::Byte, Size::OWord]                                                                               ,  [0x0F, 0x1B        ], X, ENC_MR | PREF_66, MPX;
]
"bndstx" = [
    [OperandType::Memory, Size::MemAuto, Size::Byte, Size::OWord]                                                                               ,  [0x0F, 0x1B        ], X, ENC_MR | ENC_MIB, MPX;
]
"bsf" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x0F, 0xBC        ], X, AUTO_SIZE;
]
"bsr" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x0F, 0xBD        ], X, AUTO_SIZE;
]
"bswap" = [
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0x0F, 0xC8        ], X, AUTO_REXW | SHORT_ARG;
]
"bt" = [
    [OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                 ,  [0x0F, 0xBA        ], 4, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                   ,  [0x0F, 0xA3        ], X, AUTO_SIZE | ENC_MR;
]
"btc" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Byte]                                                                      ,  [0x0F, 0xBA        ], 7, AUTO_SIZE  | EXACT_SIZE;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                       ,  [0x0F, 0xBA        ], 7, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x0F, 0xBB        ], X, AUTO_SIZE | LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x0F, 0xBB        ], X, AUTO_SIZE | ENC_MR;
]
"btr" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Byte]                                                                      ,  [0x0F, 0xBA        ], 6, AUTO_SIZE  | EXACT_SIZE;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                       ,  [0x0F, 0xBA        ], 6, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x0F, 0xB3        ], X, AUTO_SIZE | LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x0F, 0xB3        ], X, AUTO_SIZE | ENC_MR;
]
"bts" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Byte]                                                                      ,  [0x0F, 0xBA        ], 5, AUTO_SIZE  | EXACT_SIZE;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                       ,  [0x0F, 0xBA        ], 5, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x0F, 0xAB        ], X, AUTO_SIZE | LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x0F, 0xAB        ], X, AUTO_SIZE | ENC_MR;
]
"bzhi" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto, RegisterType::Legacy, Size::Auto]                                 ,  [0x02, 0xF5        ], X, VEX_OP | AUTO_REXW | ENC_MR, BMI2;
]
"cbw" = [
    []                                                                                                                                          ,  [0x98              ], X, WORD_SIZE;
]
"cdq" = [
    []                                                                                                                                          ,  [0x99              ], X;
]
"cdqe" = [
    []                                                                                                                                          ,  [0x98              ], X, WITH_REXW;
]
"clac" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xCA  ], X;
]
"clc" = [
    []                                                                                                                                          ,  [0xF8              ], X;
]
"cld" = [
    []                                                                                                                                          ,  [0xFC              ], X;
]
"clflush" = [
    [OperandType::Memory, Size::Byte]                                                                                                           ,  [0x0F, 0xAE        ], 7, DEFAULT, SSE2;
]
"clgi" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xDD  ], X, DEFAULT, VMX | AMD;
]
"cli" = [
    []                                                                                                                                          ,  [0xFA              ], X;
]
"clts" = [
    []                                                                                                                                          ,  [0x0F, 0x06        ], X;
]
"clzero" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xFC  ], X, DEFAULT, AMD;
]
"cmc" = [
    []                                                                                                                                          ,  [0xF5              ], X;
]
"cmp" = [
    [RegisterX64::rax, Size::Byte, OperandType::Immediate, Size::Byte]                                                                          ,  [0x3C              ], X;
    [RegisterType::Legacy, Size::Byte, OperandType::LegacyMemory, Size::Byte]                                                                   ,  [0x3A              ], X;
    [OperandType::LegacyMemory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                 ,  [0x80              ], 7;
    [OperandType::LegacyMemory, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                   ,  [0x38              ], X, ENC_MR;
    [RegisterX64::rax, Size::Auto, OperandType::Immediate, Size::Auto]                                                                          ,  [0x3D              ], X, AUTO_SIZE;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x3B              ], X, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Auto]                                                                 ,  [0x81              ], 7, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                 ,  [0x83              ], 7, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                   ,  [0x39              ], X, AUTO_SIZE | ENC_MR;
]
"cmpeqpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x00  ], X, PREF_66 | IMM_OP, SSE2;
]
"cmpeqps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x00  ], X, IMM_OP, SSE;
]
"cmpeqsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0xC2, 0x00  ], X, PREF_F2 | IMM_OP, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x00  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpeqss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0xC2, 0x00  ], X, PREF_F3 | IMM_OP, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x00  ], X, PREF_F3 | IMM_OP, SSE;
]
"cmplepd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x02  ], X, IMM_OP | PREF_66, SSE2;
]
"cmpleps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x02  ], X, IMM_OP, SSE;
]
"cmplesd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0xC2, 0x02  ], X, PREF_F2 | IMM_OP, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x02  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpless" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0xC2, 0x02  ], X, PREF_F3 | IMM_OP, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x02  ], X, PREF_F3 | IMM_OP, SSE;
]
"cmpltpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x01  ], X, IMM_OP | PREF_66, SSE2;
]
"cmpltps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x01  ], X, IMM_OP, SSE;
]
"cmpltsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0xC2, 0x01  ], X, PREF_F2 | IMM_OP, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x01  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpltss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0xC2, 0x01  ], X, IMM_OP | PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x01  ], X, IMM_OP | PREF_F3, SSE;
]
"cmpneqpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x04  ], X, PREF_66 | IMM_OP, SSE2;
]
"cmpneqps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x04  ], X, IMM_OP, SSE;
]
"cmpneqsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0xC2, 0x04  ], X, PREF_F2 | IMM_OP, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x04  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpneqss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0xC2, 0x04  ], X, IMM_OP | PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x04  ], X, IMM_OP | PREF_F3, SSE;
]
"cmpnlepd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x06  ], X, IMM_OP | PREF_66, SSE2;
]
"cmpnleps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x06  ], X, IMM_OP, SSE;
]
"cmpnlesd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0xC2, 0x06  ], X, PREF_F2 | IMM_OP, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x06  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpnless" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0xC2, 0x06  ], X, IMM_OP | PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x06  ], X, IMM_OP | PREF_F3, SSE;
]
"cmpnltpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x05  ], X, PREF_66 | IMM_OP, SSE2;
]
"cmpnltps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x05  ], X, IMM_OP, SSE;
]
"cmpnltsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0xC2, 0x05  ], X, IMM_OP | PREF_F2, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x05  ], X, IMM_OP | PREF_F2, SSE2;
]
"cmpnltss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0xC2, 0x05  ], X, PREF_F3 | IMM_OP, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x05  ], X, PREF_F3 | IMM_OP, SSE;
]
"cmpordpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x07  ], X, IMM_OP | PREF_66, SSE2;
]
"cmpordps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x07  ], X, IMM_OP, SSE;
]
"cmpordsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0xC2, 0x07  ], X, PREF_F2 | IMM_OP, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x07  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpordss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0xC2, 0x07  ], X, IMM_OP | PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x07  ], X, IMM_OP | PREF_F3, SSE;
]
"cmppd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x0F, 0xC2        ], X, PREF_66, SSE2;
]
"cmpps" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::MemAuto, OperandType::Immediate, Size::Byte]                                    ,  [0x0F, 0xC2        ], X, DEFAULT, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0xC2        ], X, DEFAULT, SSE;
]
"cmpsb" = [
    []                                                                                                                                          ,  [0xA6              ], X, REPE;
]
"cmpsd" = [
    []                                                                                                                                          ,  [0xA7              ], X, REPE;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x0F, 0xC2        ], X, PREF_F2, SSE2;
]
"cmpsq" = [
    []                                                                                                                                          ,  [0xA7              ], X, REPE | WITH_REXW;
]
"cmpss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::MemAuto, OperandType::Immediate, Size::Byte]                                    ,  [0x0F, 0xC2        ], X, PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0xC2        ], X, PREF_F3, SSE;
]
"cmpsw" = [
    []                                                                                                                                          ,  [0xA7              ], X, REPE | WORD_SIZE;
]
"cmpunordpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x03  ], X, PREF_66 | IMM_OP, SSE2;
]
"cmpunordps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xC2, 0x03  ], X, IMM_OP, SSE;
]
"cmpunordsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0xC2, 0x03  ], X, PREF_F2 | IMM_OP, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x03  ], X, PREF_F2 | IMM_OP, SSE2;
]
"cmpunordss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0xC2, 0x03  ], X, PREF_F3 | IMM_OP, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xC2, 0x03  ], X, PREF_F3 | IMM_OP, SSE;
]
"cmpxchg" = [
    [OperandType::Memory, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                         ,  [0x0F, 0xB0        ], X, LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                        ,  [0x0F, 0xB0        ], X, ENC_MR;
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x0F, 0xB1        ], X, AUTO_SIZE | LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x0F, 0xB1        ], X, AUTO_SIZE | ENC_MR;
]
"cmpxchg16b" = [
    [OperandType::Memory, Size::OWord]                                                                                                          ,  [0x0F, 0xC7        ], 1, LOCK | WITH_REXW;
]
"cmpxchg8b" = [
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0x0F, 0xC7        ], 1, LOCK;
]
"comisd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x2F        ], X, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x2F        ], X, PREF_66, SSE2;
]
"comiss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x2F        ], X, DEFAULT, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x2F        ], X, DEFAULT, SSE;
]
"cpu_read" = [
    []                                                                                                                                          ,  [0x0F, 0x3D        ], X, DEFAULT, CYRIX;
]
"cpu_write" = [
    []                                                                                                                                          ,  [0x0F, 0x3C        ], X, DEFAULT, CYRIX;
]
"cpuid" = [
    []                                                                                                                                          ,  [0x0F, 0xA2        ], X;
]
"cqo" = [
    []                                                                                                                                          ,  [0x99              ], X, WITH_REXW;
]
"cvtdq2pd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0xE6        ], X, PREF_F3, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xE6        ], X, PREF_F3, SSE2;
]
"cvtdq2ps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x5B        ], X, DEFAULT, SSE2;
]
"cvtpd2dq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xE6        ], X, PREF_F2, SSE2;
]
"cvtpd2pi" = [
    [RegisterType::MMX, Size::QWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x2D        ], X, PREF_66, SSE2;
]
"cvtpd2ps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x5A        ], X, PREF_66, SSE2;
]
"cvtpi2pd" = [
    [RegisterType::AVX, Size::OWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x2A        ], X, PREF_66, SSE2;
]
"cvtpi2ps" = [
    [RegisterType::AVX, Size::OWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x2A        ], X, DEFAULT, MMX | SSE;
]
"cvtps2dq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x5B        ], X, PREF_66, SSE2;
]
"cvtps2pd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x5A        ], X, DEFAULT, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x5A        ], X, DEFAULT, SSE2;
]
"cvtps2pi" = [
    [RegisterType::MMX, Size::QWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x2D        ], X, DEFAULT, SSE | MMX;
    [RegisterType::MMX, Size::QWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x2D        ], X, DEFAULT, SSE | MMX;
]
"cvtsd2si" = [
    [RegisterType::Legacy, Size::DWord, OperandType::Memory, Size::QWord]                                                                       ,  [0x0F, 0x2D        ], X, PREF_F2, SSE2;
    [RegisterType::Legacy, Size::DWord, RegisterType::AVX, Size::OWord]                                                                         ,  [0x0F, 0x2D        ], X, PREF_F2, SSE2;
    [RegisterType::Legacy, Size::QWord, OperandType::Memory, Size::QWord]                                                                       ,  [0x0F, 0x2D        ], X, WITH_REXW | PREF_F2, SSE2;
    [RegisterType::Legacy, Size::QWord, RegisterType::AVX, Size::OWord]                                                                         ,  [0x0F, 0x2D        ], X, WITH_REXW | PREF_F2, SSE2;
]
"cvtsd2ss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x5A        ], X, PREF_F2, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x5A        ], X, PREF_F2, SSE2;
]
"cvtsi2sd" = [
    [RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::DWord]                                                                    ,  [0x0F, 0x2A        ], X, PREF_F2, SSE2;
    [RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::QWord]                                                                    ,  [0x0F, 0x2A        ], X, WITH_REXW | PREF_F2, SSE2;
]
"cvtsi2ss" = [
    [RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::DWord]                                                                    ,  [0x0F, 0x2A        ], X, PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::QWord]                                                                    ,  [0x0F, 0x2A        ], X, WITH_REXW | PREF_F3, SSE;
]
"cvtss2sd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x5A        ], X, PREF_F3, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x5A        ], X, PREF_F3, SSE2;
]
"cvtss2si" = [
    [RegisterType::Legacy, Size::DWord, OperandType::Memory, Size::DWord]                                                                       ,  [0x0F, 0x2D        ], X, PREF_F3, SSE;
    [RegisterType::Legacy, Size::DWord, RegisterType::AVX, Size::OWord]                                                                         ,  [0x0F, 0x2D        ], X, PREF_F3, SSE;
    [RegisterType::Legacy, Size::QWord, OperandType::Memory, Size::DWord]                                                                       ,  [0x0F, 0x2D        ], X, WITH_REXW | PREF_F3, SSE;
    [RegisterType::Legacy, Size::QWord, RegisterType::AVX, Size::OWord]                                                                         ,  [0x0F, 0x2D        ], X, WITH_REXW | PREF_F3, SSE;
]
"cvttpd2dq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xE6        ], X, PREF_66, SSE2;
]
"cvttpd2pi" = [
    [RegisterType::MMX, Size::QWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x2C        ], X, PREF_66, SSE2;
]
"cvttps2dq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x5B        ], X, PREF_F3, SSE2;
]
"cvttps2pi" = [
    [RegisterType::MMX, Size::QWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x2C        ], X, DEFAULT, SSE | MMX;
    [RegisterType::MMX, Size::QWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x2C        ], X, DEFAULT, SSE | MMX;
]
"cvttsd2si" = [
    [RegisterType::Legacy, Size::DWord, OperandType::Memory, Size::QWord]                                                                       ,  [0x0F, 0x2C        ], X, PREF_F2, SSE2;
    [RegisterType::Legacy, Size::DWord, RegisterType::AVX, Size::OWord]                                                                         ,  [0x0F, 0x2C        ], X, PREF_F2, SSE2;
    [RegisterType::Legacy, Size::QWord, OperandType::Memory, Size::QWord]                                                                       ,  [0x0F, 0x2C        ], X, WITH_REXW | PREF_F2, SSE2;
    [RegisterType::Legacy, Size::QWord, RegisterType::AVX, Size::OWord]                                                                         ,  [0x0F, 0x2C        ], X, WITH_REXW | PREF_F2, SSE2;
]
"cvttss2si" = [
    [RegisterType::Legacy, Size::DWord, OperandType::Memory, Size::DWord]                                                                       ,  [0x0F, 0x2C        ], X, PREF_F3, SSE;
    [RegisterType::Legacy, Size::DWord, RegisterType::AVX, Size::OWord]                                                                         ,  [0x0F, 0x2C        ], X, PREF_F3, SSE;
    [RegisterType::Legacy, Size::QWord, OperandType::Memory, Size::DWord]                                                                       ,  [0x0F, 0x2C        ], X, WITH_REXW | PREF_F3, SSE;
    [RegisterType::Legacy, Size::QWord, RegisterType::AVX, Size::OWord]                                                                         ,  [0x0F, 0x2C        ], X, WITH_REXW | PREF_F3, SSE;
]
"cwd" = [
    []                                                                                                                                          ,  [0x99              ], X, WORD_SIZE;
]
"cwde" = [
    []                                                                                                                                          ,  [0x98              ], X;
]
"daa" = [
    []                                                                                                                                          ,  [0x27              ], X, X86_ONLY;
]
"das" = [
    []                                                                                                                                          ,  [0x2F              ], X, X86_ONLY;
]
"dec" = [
    [OperandType::Memory, Size::Byte]                                                                                                           ,  [0xFE              ], 1, LOCK;
    [RegisterType::Legacy, Size::Byte]                                                                                                          ,  [0xFE              ], 1;
    [OperandType::Memory, Size::Auto]                                                                                                           ,  [0xFF              ], 1, AUTO_SIZE | LOCK;
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0x48              ], 0, X86_ONLY | SHORT_ARG;
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0xFF              ], 1, AUTO_SIZE ;
]
"div" = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,  [0xF6              ], 6;
    [OperandType::LegacyMemory, Size::Auto]                                                                                                     ,  [0xF7              ], 6, AUTO_SIZE;
]
"divpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x5E        ], X, PREF_66, SSE2;
]
"divps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x5E        ], X, DEFAULT, SSE;
]
"divsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x5E        ], X, PREF_F2, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x5E        ], X, PREF_F2, SSE2;
]
"divss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x5E        ], X, PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x5E        ], X, PREF_F3, SSE;
]
"dmint" = [
    []                                                                                                                                          ,  [0x0F, 0x39        ], X, DEFAULT, CYRIX;
]
"dppd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x41  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x41  ], X, PREF_66, SSE41;
]
"dpps" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x40  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x40  ], X, PREF_66, SSE41;
]
"emms" = [
    []                                                                                                                                          ,  [0x0F, 0x77        ], X, DEFAULT, MMX;
]
"enter" = [
    [OperandType::Immediate, Size::Word, OperandType::Immediate, Size::Byte]                                                                    ,  [0xC8              ], X;
]
"extractps" = [
    [RegisterType::Legacy, Size::QWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                     ,  [0x0F, 0x3A, 0x17  ], X, WITH_REXW | ENC_MR | PREF_66, SSE41;
    [OperandType::LegacyMemory, Size::DWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                ,  [0x0F, 0x3A, 0x17  ], X, ENC_MR | PREF_66, SSE41;
]
"extrq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte, OperandType::Immediate, Size::Byte]                                    ,  [0x0F, 0x78        ], 0, PREF_66, SSE4A | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x79        ], X, PREF_66, SSE4A | AMD;
]
"f2xm1" = [
    []                                                                                                                                          ,  [0xD9, 0xF0        ], X, DEFAULT, FPU;
]
"fabs" = [
    []                                                                                                                                          ,  [0xD9, 0xE1        ], X, DEFAULT, FPU;
]
"fadd" = [
    []                                                                                                                                          ,  [0xDE, 0xC1        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xD8, 0xC0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xD8, 0xC0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDC, 0xC0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDC, 0xC0        ], X, SHORT_ARG, FPU;
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xD8              ], 0, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDC              ], 0, EXACT_SIZE, FPU;
]
"faddp" = [
    []                                                                                                                                          ,  [0xDE, 0xC1        ], X, DEFAULT, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDE, 0xC0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDE, 0xC0        ], X, SHORT_ARG, FPU;
]
"fbld" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0xDF              ], 4, DEFAULT, FPU;
]
"fbstp" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0xDF              ], 6, DEFAULT, FPU;
]
"fchs" = [
    []                                                                                                                                          ,  [0xD9, 0xE0        ], X, DEFAULT, FPU;
]
"fclex" = [
    []                                                                                                                                          ,  [0x9B, 0xDB, 0xE2  ], X, DEFAULT, FPU;
]
"fcmovb" = [
    []                                                                                                                                          ,  [0xDA, 0xC1        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDA, 0xC0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDA, 0xC0        ], X, SHORT_ARG, FPU;
]
"fcmovbe" = [
    []                                                                                                                                          ,  [0xDA, 0xD1        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDA, 0xD0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDA, 0xD0        ], X, SHORT_ARG, FPU;
]
"fcmove" = [
    []                                                                                                                                          ,  [0xDA, 0xC9        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDA, 0xC8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDA, 0xC8        ], X, SHORT_ARG, FPU;
]
"fcmovnb" = [
    []                                                                                                                                          ,  [0xDB, 0xC1        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDB, 0xC0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDB, 0xC0        ], X, SHORT_ARG, FPU;
]
"fcmovnbe" = [
    []                                                                                                                                          ,  [0xDB, 0xD1        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDB, 0xD0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDB, 0xD0        ], X, SHORT_ARG, FPU;
]
"fcmovne" = [
    []                                                                                                                                          ,  [0xDB, 0xC9        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDB, 0xC8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDB, 0xC8        ], X, SHORT_ARG, FPU;
]
"fcmovnu" = [
    []                                                                                                                                          ,  [0xDB, 0xD9        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDB, 0xD8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDB, 0xD8        ], X, SHORT_ARG, FPU;
]
"fcmovu" = [
    []                                                                                                                                          ,  [0xDA, 0xD9        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDA, 0xD8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDA, 0xD8        ], X, SHORT_ARG, FPU;
]
"fcom" = [
    []                                                                                                                                          ,  [0xD8, 0xD1        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xD8, 0xD0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xD8, 0xD0        ], X, SHORT_ARG, FPU;
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xD8              ], 2, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDC              ], 2, EXACT_SIZE, FPU;
]
"fcomi" = [
    []                                                                                                                                          ,  [0xDB, 0xF1        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDB, 0xF0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDB, 0xF0        ], X, SHORT_ARG, FPU;
]
"fcomip" = [
    []                                                                                                                                          ,  [0xDF, 0xF1        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDF, 0xF0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDF, 0xF0        ], X, SHORT_ARG, FPU;
]
"fcomp" = [
    []                                                                                                                                          ,  [0xD8, 0xD9        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xD8, 0xD8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xD8, 0xD8        ], X, SHORT_ARG, FPU;
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xD8              ], 3, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDC              ], 3, EXACT_SIZE, FPU;
]
"fcompp" = [
    []                                                                                                                                          ,  [0xDE, 0xD9        ], X, DEFAULT, FPU;
]
"fcos" = [
    []                                                                                                                                          ,  [0xD9, 0xFF        ], X, DEFAULT, FPU;
]
"fdecstp" = [
    []                                                                                                                                          ,  [0xD9, 0xF6        ], X, DEFAULT, FPU;
]
"fdisi" = [
    []                                                                                                                                          ,  [0x9B, 0xDB, 0xE1  ], X, DEFAULT, FPU;
]
"fdiv" = [
    []                                                                                                                                          ,  [0xDE, 0xF9        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xD8, 0xF0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xD8, 0xF0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDC, 0xF8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDC, 0xF8        ], X, SHORT_ARG, FPU;
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xD8              ], 6, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDC              ], 6, EXACT_SIZE, FPU;
]
"fdivp" = [
    []                                                                                                                                          ,  [0xDE, 0xF9        ], X, DEFAULT, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDE, 0xF8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDE, 0xF8        ], X, SHORT_ARG, FPU;
]
"fdivr" = [
    []                                                                                                                                          ,  [0xDE, 0xF1        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xD8, 0xF8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xD8, 0xF8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDC, 0xF0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDC, 0xF0        ], X, SHORT_ARG, FPU;
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xD8              ], 7, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDC              ], 7, EXACT_SIZE, FPU;
]
"fdivrp" = [
    []                                                                                                                                          ,  [0xDE, 0xF1        ], X, DEFAULT, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDE, 0xF0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDE, 0xF0        ], X, SHORT_ARG, FPU;
]
"femms" = [
    []                                                                                                                                          ,  [0x0F, 0x0E        ], X, DEFAULT, TDNOW;
]
"feni" = [
    []                                                                                                                                          ,  [0x9B, 0xDB, 0xE0  ], X, DEFAULT, FPU;
]
"ffree" = [
    []                                                                                                                                          ,  [0xDD, 0xC1        ], X, DEFAULT, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDD, 0xC0        ], X, SHORT_ARG, FPU;
]
"fiadd" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xDA              ], 0, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xDE              ], 0, DEFAULT, FPU;
]
"ficom" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xDA              ], 2, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xDE              ], 2, DEFAULT, FPU;
]
"ficomp" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xDA              ], 3, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xDE              ], 3, DEFAULT, FPU;
]
"fidiv" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xDA              ], 6, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xDE              ], 6, DEFAULT, FPU;
]
"fidivr" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xDA              ], 7, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xDE              ], 7, DEFAULT, FPU;
]
"fild" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xDB              ], 0, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDF              ], 5, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xDF              ], 0, DEFAULT, FPU;
]
"fimul" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xDA              ], 1, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xDE              ], 1, DEFAULT, FPU;
]
"fincstp" = [
    []                                                                                                                                          ,  [0xD9, 0xF7        ], X, DEFAULT, FPU;
]
"finit" = [
    []                                                                                                                                          ,  [0x9B, 0xDB, 0xE3  ], X, DEFAULT, FPU;
]
"fist" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xDB              ], 2, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xDF              ], 2, DEFAULT, FPU;
]
"fistp" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xDB              ], 3, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDF              ], 7, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xDF              ], 3, DEFAULT, FPU;
]
"fisttp" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xDB              ], 1, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDD              ], 1, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xDF              ], 1, DEFAULT, FPU;
]
"fisub" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xDA              ], 4, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xDE              ], 4, DEFAULT, FPU;
]
"fisubr" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xDA              ], 5, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xDE              ], 5, DEFAULT, FPU;
]
"fld" = [
    []                                                                                                                                          ,  [0xD9, 0xC1        ], X, DEFAULT, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xD9, 0xC0        ], X, SHORT_ARG, FPU;
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xD9              ], 0, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::PWord]                                                                                                          ,  [0xDB              ], 5, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDD              ], 0, EXACT_SIZE, FPU;
]
"fld1" = [
    []                                                                                                                                          ,  [0xD9, 0xE8        ], X, DEFAULT, FPU;
]
"fldcw" = [
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xD9              ], 5, DEFAULT, FPU;
]
"fldenv" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0xD9              ], 4, DEFAULT, FPU;
]
"fldl2e" = [
    []                                                                                                                                          ,  [0xD9, 0xEA        ], X, DEFAULT, FPU;
]
"fldl2t" = [
    []                                                                                                                                          ,  [0xD9, 0xE9        ], X, DEFAULT, FPU;
]
"fldlg2" = [
    []                                                                                                                                          ,  [0xD9, 0xEC        ], X, DEFAULT, FPU;
]
"fldln2" = [
    []                                                                                                                                          ,  [0xD9, 0xED        ], X, DEFAULT, FPU;
]
"fldpi" = [
    []                                                                                                                                          ,  [0xD9, 0xEB        ], X, DEFAULT, FPU;
]
"fldz" = [
    []                                                                                                                                          ,  [0xD9, 0xEE        ], X, DEFAULT, FPU;
]
"fmul" = [
    []                                                                                                                                          ,  [0xDE, 0xC9        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xD8, 0xC8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xD8, 0xC8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDC, 0xC8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDC, 0xC8        ], X, SHORT_ARG, FPU;
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xD8              ], 1, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDC              ], 1, EXACT_SIZE, FPU;
]
"fmulp" = [
    []                                                                                                                                          ,  [0xDE, 0xC9        ], X, DEFAULT, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDE, 0xC8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDE, 0xC8        ], X, SHORT_ARG, FPU;
]
"fnclex" = [
    []                                                                                                                                          ,  [0xDB, 0xE2        ], X, DEFAULT, FPU;
]
"fndisi" = [
    []                                                                                                                                          ,  [0xDB, 0xE1        ], X, DEFAULT, FPU;
]
"fneni" = [
    []                                                                                                                                          ,  [0xDB, 0xE0        ], X, DEFAULT, FPU;
]
"fninit" = [
    []                                                                                                                                          ,  [0xDB, 0xE3        ], X, DEFAULT, FPU;
]
"fnop" = [
    []                                                                                                                                          ,  [0xD9, 0xD0        ], X, DEFAULT, FPU;
]
"fnsave" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0xDD              ], 6, DEFAULT, FPU;
]
"fnstcw" = [
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xD9              ], 7, DEFAULT, FPU;
]
"fnstenv" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0xD9              ], 6, DEFAULT, FPU;
]
"fnstsw" = [
    [RegisterX64::rax, Size::Word]                                                                                                              ,  [0xDF, 0xE0        ], X, DEFAULT, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0xDD              ], 7, DEFAULT, FPU;
]
"fpatan" = [
    []                                                                                                                                          ,  [0xD9, 0xF3        ], X, DEFAULT, FPU;
]
"fprem" = [
    []                                                                                                                                          ,  [0xD9, 0xF8        ], X, DEFAULT, FPU;
]
"fprem1" = [
    []                                                                                                                                          ,  [0xD9, 0xF5        ], X, DEFAULT, FPU;
]
"fptan" = [
    []                                                                                                                                          ,  [0xD9, 0xF2        ], X, DEFAULT, FPU;
]
"frndint" = [
    []                                                                                                                                          ,  [0xD9, 0xFC        ], X, DEFAULT, FPU;
]
"frstor" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0xDD              ], 4, DEFAULT, FPU;
]
"fsave" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x9B, 0xDD        ], 6, DEFAULT, FPU;
]
"fscale" = [
    []                                                                                                                                          ,  [0xD9, 0xFD        ], X, DEFAULT, FPU;
]
"fsetpm" = [
    []                                                                                                                                          ,  [0xDB, 0xE4        ], X, DEFAULT, FPU;
]
"fsin" = [
    []                                                                                                                                          ,  [0xD9, 0xFE        ], X, DEFAULT, FPU;
]
"fsincos" = [
    []                                                                                                                                          ,  [0xD9, 0xFB        ], X, DEFAULT, FPU;
]
"fsqrt" = [
    []                                                                                                                                          ,  [0xD9, 0xFA        ], X, DEFAULT, FPU;
]
"fst" = [
    []                                                                                                                                          ,  [0xDD, 0xD1        ], X, DEFAULT, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDD, 0xD0        ], X, SHORT_ARG, FPU;
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xD9              ], 2, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDD              ], 2, EXACT_SIZE, FPU;
]
"fstcw" = [
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0x9B, 0xD9        ], 7, DEFAULT, FPU;
]
"fstenv" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x9B, 0xD9        ], 6, DEFAULT, FPU;
]
"fstp" = [
    []                                                                                                                                          ,  [0xDD, 0xD9        ], X, DEFAULT, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDD, 0xD8        ], X, SHORT_ARG, FPU;
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xD9              ], 3, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::PWord]                                                                                                          ,  [0xDB              ], 7, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDD              ], 3, EXACT_SIZE, FPU;
]
"fstsw" = [
    [RegisterX64::rax, Size::Word]                                                                                                              ,  [0x9B, 0xDF, 0xE0  ], X, DEFAULT, FPU;
    [OperandType::Memory, Size::Word]                                                                                                           ,  [0x9B, 0xDD        ], 7, DEFAULT, FPU;
]
"fsub" = [
    []                                                                                                                                          ,  [0xDE, 0xE9        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xD8, 0xE0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xD8, 0xE0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDC, 0xE8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDC, 0xE8        ], X, SHORT_ARG, FPU;
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xD8              ], 4, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDC              ], 4, EXACT_SIZE, FPU;
]
"fsubp" = [
    []                                                                                                                                          ,  [0xDE, 0xE9        ], X, DEFAULT, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDE, 0xE8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDE, 0xE8        ], X, SHORT_ARG, FPU;
]
"fsubr" = [
    []                                                                                                                                          ,  [0xDE, 0xE1        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xD8, 0xE8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xD8, 0xE8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDC, 0xE0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDC, 0xE0        ], X, SHORT_ARG, FPU;
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xD8              ], 5, EXACT_SIZE, FPU;
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0xDC              ], 5, EXACT_SIZE, FPU;
]
"fsubrp" = [
    []                                                                                                                                          ,  [0xDE, 0xE1        ], X, DEFAULT, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDE, 0xE0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xDE, 0xE0        ], X, SHORT_ARG, FPU;
]
"ftst" = [
    []                                                                                                                                          ,  [0xD9, 0xE4        ], X, DEFAULT, FPU;
]
"fucom" = [
    []                                                                                                                                          ,  [0xDD, 0xE1        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDD, 0xE0        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDD, 0xE0        ], X, SHORT_ARG, FPU;
]
"fucomi" = [
    []                                                                                                                                          ,  [0xDB, 0xE9        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDB, 0xE8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDB, 0xE8        ], X, SHORT_ARG, FPU;
]
"fucomip" = [
    []                                                                                                                                          ,  [0xDF, 0xE9        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDF, 0xE8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDF, 0xE8        ], X, SHORT_ARG, FPU;
]
"fucomp" = [
    []                                                                                                                                          ,  [0xDD, 0xE9        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xDD, 0xE8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xDD, 0xE8        ], X, SHORT_ARG, FPU;
]
"fucompp" = [
    []                                                                                                                                          ,  [0xDA, 0xE9        ], X, DEFAULT, FPU;
]
"fwait" = [
    []                                                                                                                                          ,  [0x9B              ], X;
]
"fxam" = [
    []                                                                                                                                          ,  [0xD9, 0xE5        ], X, DEFAULT, FPU;
]
"fxch" = [
    []                                                                                                                                          ,  [0xD9, 0xC9        ], X, DEFAULT, FPU;
    [ControlRegister::st0, Size::PWord, Size::FWord, Size::PWord]                                                                               ,  [0xD9, 0xC8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord]                                                                                                                  ,  [0xD9, 0xC8        ], X, SHORT_ARG, FPU;
    [Size::FWord, Size::PWord, ControlRegister::st0, Size::PWord]                                                                               ,  [0xD9, 0xC8        ], X, SHORT_ARG, FPU;
]
"fxrstor" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xAE        ], 1, DEFAULT, SSE | FPU;
]
"fxrstor64" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xAE        ], 1, WITH_REXW, FPU | SSE;
]
"fxsave" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xAE        ], 0, DEFAULT, FPU | SSE;
]
"fxsave64" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xAE        ], 0, WITH_REXW, SSE | FPU;
]
"fxtract" = [
    []                                                                                                                                          ,  [0xD9, 0xF4        ], X, DEFAULT, FPU;
]
"fyl2x" = [
    []                                                                                                                                          ,  [0xD9, 0xF1        ], X, DEFAULT, FPU;
]
"fyl2xp1" = [
    []                                                                                                                                          ,  [0xD9, 0xF9        ], X, DEFAULT, FPU;
]
"getsec" = [
    []                                                                                                                                          ,  [0x0F, 0x37        ], X;
]
"haddpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x7C        ], X, PREF_66, SSE3;
]
"haddps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x7C        ], X, PREF_F2, SSE3;
]
"hlt" = [
    []                                                                                                                                          ,  [0xF4              ], X;
]
"hsubpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x7D        ], X, PREF_66, SSE3;
]
"hsubps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x7D        ], X, PREF_F2, SSE3;
]
"icebp" = [
    []                                                                                                                                          ,  [0xF1              ], X;
]
"idiv" = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,  [0xF6              ], 7;
    [OperandType::LegacyMemory, Size::Auto]                                                                                                     ,  [0xF7              ], 7, AUTO_SIZE;
]
"inc" = [
    [OperandType::Memory, Size::Byte]                                                                                                           ,  [0xFE              ], 0, LOCK;
    [RegisterType::Legacy, Size::Byte]                                                                                                          ,  [0xFE              ], 0;
    [OperandType::Memory, Size::Auto]                                                                                                           ,  [0xFF              ], 0, AUTO_SIZE | LOCK;
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0x40              ], 0, X86_ONLY | SHORT_ARG;
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0xFF              ], 0, AUTO_SIZE ;
]
"insb" = [
    []                                                                                                                                          ,  [0x6C              ], X, REP;
]
"insd" = [
    []                                                                                                                                          ,  [0x6D              ], X, REP;
]
"insertps" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x21  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x21  ], X, PREF_66, SSE41;
]
"insertq" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x79        ], X, PREF_F2, SSE4A | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte, OperandType::Immediate, Size::Byte]    ,  [0x0F, 0x78        ], X, PREF_F2, AMD | SSE4A;
]
"insw" = [
    []                                                                                                                                          ,  [0x6D              ], X, WORD_SIZE | REP;
]
"int" = [
    [OperandType::Immediate, Size::Byte]                                                                                                        ,  [0xCD              ], X;
]
"into" = [
    []                                                                                                                                          ,  [0xCE              ], X, X86_ONLY;
]
"int01" = [
    []                                                                                                                                          ,  [0xF1              ], X;
]
"int03" = [
    []                                                                                                                                          ,  [0xCC              ], X;
]
"int1" = [
    []                                                                                                                                          ,  [0xF1              ], X;
]
"int3" = [
    []                                                                                                                                          ,  [0xCC              ], X;
]
"invd" = [
    []                                                                                                                                          ,  [0x0F, 0x08        ], X;
]
"invept" = [
    [RegisterType::Legacy, Size::QWord, OperandType::Memory, Size::OWord]                                                                       ,  [0x0F, 0x38, 0x80  ], X, PREF_66, VMX;
]
"invlpg" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0x01        ], 7;
]
"invlpga" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xDF  ], X, DEFAULT, AMD;
    [RegisterX64::rax, Size::QWord, RegisterX64::rcx, Size::DWord]                                                                              ,  [0x0F, 0x01, 0xDF  ], X, DEFAULT, AMD;
]
"invpcid" = [
    [RegisterType::Legacy, Size::QWord, OperandType::Memory, Size::OWord]                                                                       ,  [0x0F, 0x38, 0x82  ], X, PREF_66, INVPCID;
]
"invvpid" = [
    [RegisterType::Legacy, Size::QWord, OperandType::Memory, Size::OWord]                                                                       ,  [0x0F, 0x38, 0x81  ], X, PREF_66, VMX;
]
"iret" = [
    []                                                                                                                                          ,  [0xCF              ], X;
]
"iretd" = [
    []                                                                                                                                          ,  [0xCF              ], X;
]
"iretq" = [
    []                                                                                                                                          ,  [0xCF              ], X, WITH_REXW;
]
"iretw" = [
    []                                                                                                                                          ,  [0xCF              ], X, WORD_SIZE;
]
"jecxz" = [
    [Size::OWord, Size::Byte]                                                                                                                   ,  [0xE3              ], X, PREF_67;
]
"jrcxz" = [
    [Size::OWord, Size::Byte]                                                                                                                   ,  [0xE3              ], X;
]
"lahf" = [
    []                                                                                                                                          ,  [0x9F              ], X;
]
"lar" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::Word]                                                                         ,  [0x0F, 0x02        ], X, AUTO_SIZE;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x0F, 0x02        ], X, AUTO_SIZE;
]
"lddqu" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::OWord]                                                                          ,  [0x0F, 0xF0        ], X, PREF_F2, SSE3;
]
"ldmxcsr" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0x0F, 0xAE        ], 2, DEFAULT, SSE;
]
"lds" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::MemAuto]                                                                      ,  [0xC5              ], X, AUTO_SIZE | X86_ONLY;
]
"lea" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::MemAuto]                                                                      ,  [0x8D              ], X, AUTO_SIZE;
]
"leave" = [
    []                                                                                                                                          ,  [0xC9              ], X;
]
"les" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::MemAuto]                                                                      ,  [0xC4              ], X, AUTO_SIZE | X86_ONLY;
]
"lfence" = [
    []                                                                                                                                          ,  [0x0F, 0xAE, 0xE8  ], X, DEFAULT, AMD;
]
"lfs" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::MemAuto]                                                                      ,  [0x0F, 0xB4        ], X, AUTO_SIZE;
]
"lgdt" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0x01        ], 2;
]
"lgs" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::MemAuto]                                                                      ,  [0x0F, 0xB5        ], X, AUTO_SIZE;
]
"lidt" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0x01        ], 3;
]
"lldt" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0x00        ], 2;
    [RegisterType::Legacy, Size::Word]                                                                                                          ,  [0x0F, 0x00        ], 2;
]
"llwpcb" = [
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0x09, 0x12        ], 0, XOP_OP | AUTO_REXW, AMD;
]
"lmsw" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0x01        ], 6;
    [RegisterType::Legacy, Size::Word]                                                                                                          ,  [0x0F, 0x01        ], 6;
]
"lodsb" = [
    []                                                                                                                                          ,  [0xAC              ], X, REP;
]
"lodsd" = [
    []                                                                                                                                          ,  [0xAD              ], X, REP;
]
"lodsq" = [
    []                                                                                                                                          ,  [0xAD              ], X, WITH_REXW | REP;
]
"lodsw" = [
    []                                                                                                                                          ,  [0xAD              ], X, WORD_SIZE | REP;
]
"loop" = [
    [Size::OWord, Size::Byte]                                                                                                                   ,  [0xE2              ], X;
]
"loope" = [
    [Size::OWord, Size::Byte]                                                                                                                   ,  [0xE1              ], X;
]
"loopne" = [
    [Size::OWord, Size::Byte]                                                                                                                   ,  [0xE0              ], X;
]
"loopnz" = [
    [Size::OWord, Size::Byte]                                                                                                                   ,  [0xE0              ], X;
]
"loopz" = [
    [Size::OWord, Size::Byte]                                                                                                                   ,  [0xE1              ], X;
]
"lsl" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::Word]                                                                         ,  [0x0F, 0x03        ], X, AUTO_SIZE;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x0F, 0x03        ], X, AUTO_SIZE;
]
"lss" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::MemAuto]                                                                      ,  [0x0F, 0xB2        ], X, AUTO_SIZE;
]
"ltr" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0x00        ], 3;
    [RegisterType::Legacy, Size::Word]                                                                                                          ,  [0x0F, 0x00        ], 3;
]
"lwpins" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::DWord]                              ,  [0x10, 0x12        ], 0, XOP_OP | AUTO_REXW | ENC_VM, AMD;
]
"lwpval" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::DWord]                              ,  [0x10, 0x12        ], 1, XOP_OP | AUTO_REXW | ENC_VM, AMD;
]
"lzcnt" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x0F, 0xBD        ], X, AUTO_SIZE | PREF_F3, AMD;
]
"maskmovdqu" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xF7        ], X, PREF_66, SSE2;
]
"maskmovq" = [
    [RegisterType::MMX, Size::QWord, RegisterType::MMX, Size::QWord]                                                                            ,  [0x0F, 0xF7        ], X, DEFAULT, MMX;
]
"maxpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x5F        ], X, PREF_66, SSE2;
]
"maxps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x5F        ], X, DEFAULT, SSE;
]
"maxsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x5F        ], X, PREF_F2, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x5F        ], X, PREF_F2, SSE2;
]
"maxss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x5F        ], X, PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x5F        ], X, PREF_F3, SSE;
]
"mfence" = [
    []                                                                                                                                          ,  [0x0F, 0xAE, 0xF0  ], X, DEFAULT, AMD;
]
"minpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x5D        ], X, PREF_66, SSE2;
]
"minps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x5D        ], X, DEFAULT, SSE;
]
"minsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x5D        ], X, PREF_F2, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x5D        ], X, PREF_F2, SSE2;
]
"minss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x5D        ], X, PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x5D        ], X, PREF_F3, SSE;
]
"monitor" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xC8  ], X;
    [RegisterX64::rax, Size::QWord, RegisterX64::rcx, Size::DWord, RegisterX64::rdx, Size::DWord]                                               ,  [0x0F, 0x01, 0xC8  ], X;
]
"monitorx" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xFA  ], X, DEFAULT, AMD;
    [RegisterX64::rax, Size::Auto, RegisterX64::rcx, Size::DWord, RegisterX64::rdx, Size::DWord]                                                ,  [0x0F, 0x01, 0xFA  ], X, DEFAULT, AMD;
]
"montmul" = [
    []                                                                                                                                          ,  [0x0F, 0xA6, 0xC0  ], X, PREF_F3, CYRIX;
]
"movapd" = [
    [OperandType::Memory, Size::OWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x29        ], X, ENC_MR | PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::OWord]                                                                          ,  [0x0F, 0x28        ], X, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x28        ], X, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x29        ], X, ENC_MR | PREF_66, SSE2;
]
"movaps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x28        ], X, DEFAULT, SSE;
    [Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                                                   ,  [0x0F, 0x29        ], X, ENC_MR, SSE;
]
"movbe" = [
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x0F, 0x38, 0xF1  ], X, AUTO_SIZE | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::Auto]                                                                         ,  [0x0F, 0x38, 0xF0  ], X, AUTO_SIZE;
]
"movd" = [
    [OperandType::Memory, Size::DWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x7E        ], X, ENC_MR | PREF_66, SSE2;
    [RegisterType::MMX, Size::QWord, OperandType::LegacyMemory, Size::DWord]                                                                    ,  [0x0F, 0x6E        ], X, DEFAULT, MMX;
    [RegisterType::MMX, Size::QWord, OperandType::LegacyMemory, Size::QWord]                                                                    ,  [0x0F, 0x6E        ], X, WITH_REXW, MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x6E        ], X, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::DWord]                                                                    ,  [0x0F, 0x6E        ], X, PREF_66, SSE2;
    [OperandType::LegacyMemory, Size::DWord, RegisterType::MMX, Size::QWord]                                                                    ,  [0x0F, 0x7E        ], X, ENC_MR, MMX;
    [OperandType::LegacyMemory, Size::DWord, RegisterType::AVX, Size::OWord]                                                                    ,  [0x0F, 0x7E        ], X, ENC_MR | PREF_66, SSE2;
    [OperandType::LegacyMemory, Size::QWord, RegisterType::MMX, Size::QWord]                                                                    ,  [0x0F, 0x7E        ], X, WITH_REXW | ENC_MR, MMX;
]
"movddup" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x12        ], X, PREF_F2, SSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x12        ], X, PREF_F2, SSE3;
]
"movdq2q" = [
    [RegisterType::MMX, Size::QWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xD6        ], X, PREF_F2, SSE2;
]
"movdqa" = [
    [OperandType::Memory, Size::OWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x7F        ], X, ENC_MR | PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::OWord]                                                                          ,  [0x0F, 0x6F        ], X, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x6F        ], X, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x7F        ], X, ENC_MR | PREF_66, SSE2;
]
"movdqu" = [
    [OperandType::Memory, Size::OWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x7F        ], X, ENC_MR | PREF_F3, SSE2;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::OWord]                                                                          ,  [0x0F, 0x6F        ], X, PREF_F3, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x6F        ], X, PREF_F3, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x7F        ], X, ENC_MR | PREF_F3, SSE2;
]
"movhlps" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x12        ], X, DEFAULT, SSE;
]
"movhpd" = [
    [OperandType::Memory, Size::MemAuto, RegisterType::AVX, Size::OWord]                                                                        ,  [0x0F, 0x17        ], X, ENC_MR | PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::MemAuto]                                                                        ,  [0x0F, 0x16        ], X, PREF_66, SSE2;
]
"movhps" = [
    [OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x17        ], X, ENC_MR, SSE;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x16        ], X, DEFAULT, SSE;
]
"movlhps" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x16        ], X, DEFAULT, SSE;
]
"movlpd" = [
    [OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x13        ], X, ENC_MR | PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x12        ], X, PREF_66, SSE2;
]
"movlps" = [
    [OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x13        ], X, ENC_MR, SSE;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x12        ], X, DEFAULT, SSE;
]
"movmskpd" = [
    [RegisterType::Legacy, Size::DWord, RegisterType::AVX, Size::OWord]                                                                         ,  [0x0F, 0x50        ], X, PREF_66, SSE2;
    [RegisterType::Legacy, Size::QWord, RegisterType::AVX, Size::OWord]                                                                         ,  [0x0F, 0x50        ], X, WITH_REXW | PREF_66, SSE2;
]
"movmskps" = [
    [RegisterType::Legacy, Size::DWord, RegisterType::AVX, Size::OWord]                                                                         ,  [0x0F, 0x50        ], X, DEFAULT, SSE;
    [RegisterType::Legacy, Size::QWord, RegisterType::AVX, Size::OWord]                                                                         ,  [0x0F, 0x50        ], X, WITH_REXW, SSE;
]
"movntdq" = [
    [OperandType::Memory, Size::OWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0xE7        ], X, ENC_MR | PREF_66, SSE2;
]
"movntdqa" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::OWord]                                                                          ,  [0x0F, 0x38, 0x2A  ], X, PREF_66, SSE41;
]
"movnti" = [
    [OperandType::Memory, Size::DWord, RegisterType::Legacy, Size::DWord]                                                                       ,  [0x0F, 0xC3        ], X, ENC_MR;
    [OperandType::Memory, Size::QWord, RegisterType::Legacy, Size::QWord]                                                                       ,  [0x0F, 0xC3        ], X, WITH_REXW | ENC_MR;
]
"movntpd" = [
    [OperandType::Memory, Size::OWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x2B        ], X, ENC_MR | PREF_66, SSE2;
]
"movntps" = [
    [OperandType::Memory, Size::OWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x2B        ], X, ENC_MR, SSE;
]
"movntq" = [
    [OperandType::Memory, Size::QWord, RegisterType::MMX, Size::QWord]                                                                          ,  [0x0F, 0xE7        ], X, ENC_MR, MMX;
]
"movntsd" = [
    [OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x2B        ], X, ENC_MR | PREF_F2, AMD | SSE4A;
]
"movntss" = [
    [OperandType::Memory, Size::DWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x2B        ], X, ENC_MR | PREF_F3, SSE4A | AMD;
]
"movq" = [
    [OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0xD6        ], X, ENC_MR | PREF_66, SSE2;
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x6F        ], X, DEFAULT, MMX;
    [RegisterType::MMX, Size::QWord, OperandType::LegacyMemory, Size::QWord]                                                                    ,  [0x0F, 0x6E        ], X, WITH_REXW, MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x7E        ], X, PREF_F3, SSE2;
    [RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::QWord]                                                                    ,  [0x0F, 0x6E        ], X, WITH_REXW | PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x7E        ], X, PREF_F3, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0xD6        ], X, ENC_MR | PREF_66, SSE2;
    [OperandType::MMXMemory, Size::QWord, RegisterType::MMX, Size::QWord]                                                                       ,  [0x0F, 0x7F        ], X, ENC_MR, MMX;
    [OperandType::LegacyMemory, Size::QWord, RegisterType::MMX, Size::QWord]                                                                    ,  [0x0F, 0x7E        ], X, WITH_REXW | ENC_MR, MMX;
    [OperandType::LegacyMemory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                    ,  [0x0F, 0x7E        ], X, WITH_REXW | ENC_MR | PREF_66, SSE2;
]
"movq2dq" = [
    [RegisterType::AVX, Size::OWord, RegisterType::MMX, Size::QWord]                                                                            ,  [0x0F, 0xD6        ], X, PREF_F3, SSE2;
]
"movsb" = [
    []                                                                                                                                          ,  [0xA4              ], X, REP;
]
"movsd" = [
    []                                                                                                                                          ,  [0xA5              ], X, REP;
    [OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x11        ], X, ENC_MR | PREF_F2, SSE2;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x10        ], X, PREF_F2, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x10        ], X, PREF_F2, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x11        ], X, ENC_MR | PREF_F2, SSE2;
]
"movshdup" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x16        ], X, PREF_F3, SSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x16        ], X, PREF_F3, SSE3;
]
"movsldup" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x12        ], X, PREF_F3, SSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x12        ], X, PREF_F3, SSE3;
]
"movsq" = [
    []                                                                                                                                          ,  [0xA5              ], X, WITH_REXW | REP;
]
"movss" = [
    [OperandType::Memory, Size::DWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x11        ], X, ENC_MR | PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x10        ], X, PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x10        ], X, PREF_F3, SSE;
]
"movsw" = [
    []                                                                                                                                          ,  [0xA5              ], X, WORD_SIZE | REP;
]
"movsx" = [
    [RegisterType::Legacy, Size::QWord, OperandType::LegacyMemory, Size::DWord]                                                                 ,  [0x63              ], X, WITH_REXW;
    [RegisterType::Legacy, Size::Word, OperandType::Memory, Size::Byte]                                                                         ,  [0x0F, 0xBE        ], X, WORD_SIZE;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Byte]                                                                   ,  [0x0F, 0xBE        ], X, AUTO_SIZE;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Word]                                                                   ,  [0x0F, 0xBF        ], X, AUTO_REXW | EXACT_SIZE;
]
"movsxd" = [
    [RegisterType::Legacy, Size::QWord, OperandType::LegacyMemory, Size::DWord]                                                                 ,  [0x63              ], X, WITH_REXW;
]
"movupd" = [
    [OperandType::Memory, Size::OWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x0F, 0x11        ], X, ENC_MR | PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::OWord]                                                                          ,  [0x0F, 0x10        ], X, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x10        ], X, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x11        ], X, ENC_MR | PREF_66, SSE2;
]
"movups" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x10        ], X, DEFAULT, SSE;
    [Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                                                   ,  [0x0F, 0x11        ], X, ENC_MR, SSE;
]
"movzx" = [
    [RegisterType::Legacy, Size::Word, OperandType::Memory, Size::Byte]                                                                         ,  [0x0F, 0xB6        ], X, WORD_SIZE;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Byte]                                                                   ,  [0x0F, 0xB6        ], X, AUTO_SIZE;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Word]                                                                   ,  [0x0F, 0xB7        ], X, AUTO_REXW | EXACT_SIZE;
]
"mpsadbw" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x42  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x42  ], X, PREF_66, SSE41;
]
"mul" = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,  [0xF6              ], 4;
    [OperandType::LegacyMemory, Size::Auto]                                                                                                     ,  [0xF7              ], 4, AUTO_SIZE;
]
"mulpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x59        ], X, PREF_66, SSE2;
]
"mulps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x59        ], X, DEFAULT, SSE;
]
"mulsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x59        ], X, PREF_F2, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x59        ], X, PREF_F2, SSE2;
]
"mulss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x59        ], X, PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x59        ], X, PREF_F3, SSE;
]
"mulx" = [
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                 ,  [0x02, 0xF6        ], X, VEX_OP | AUTO_REXW | PREF_F2, BMI2;
]
"mwait" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xC9  ], X;
    [RegisterX64::rax, Size::DWord, RegisterX64::rcx, Size::DWord]                                                                              ,  [0x0F, 0x01, 0xC9  ], X;
]
"mwaitx" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xFB  ], X, DEFAULT, AMD;
    [RegisterX64::rax, Size::DWord, RegisterX64::rcx, Size::DWord]                                                                              ,  [0x0F, 0x01, 0xFB  ], X, DEFAULT, AMD;
]
"neg" = [
    [OperandType::Memory, Size::Byte]                                                                                                           ,  [0xF6              ], 3, LOCK;
    [RegisterType::Legacy, Size::Byte]                                                                                                          ,  [0xF6              ], 3;
    [OperandType::Memory, Size::Auto]                                                                                                           ,  [0xF7              ], 3, AUTO_SIZE | LOCK;
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0xF7              ], 3, AUTO_SIZE ;
]
"nop" = [
    []                                                                                                                                          ,  [0x90              ], X;
    [OperandType::LegacyMemory, Size::Auto]                                                                                                     ,  [0x0F, 0x1F        ], 0, AUTO_SIZE;
]
"not" = [
    [OperandType::Memory, Size::Byte]                                                                                                           ,  [0xF6              ], 2, LOCK;
    [RegisterType::Legacy, Size::Byte]                                                                                                          ,  [0xF6              ], 2;
    [OperandType::Memory, Size::Auto]                                                                                                           ,  [0xF7              ], 2, AUTO_SIZE | LOCK;
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0xF7              ], 2, AUTO_SIZE ;
]
"or" = [
    [RegisterX64::rax, Size::Byte, OperandType::Immediate, Size::Byte]                                                                          ,  [0x0C              ], X;
    [OperandType::Memory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                       ,  [0x80              ], 1, LOCK;
    [OperandType::Memory, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                         ,  [0x08              ], X, LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::Immediate, Size::Byte]                                                                      ,  [0x80              ], 1;
    [RegisterType::Legacy, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                        ,  [0x08              ], X, ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::LegacyMemory, Size::Byte]                                                                   ,  [0x0A              ], X;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Byte]                                                                      ,  [0x83              ], 1, AUTO_SIZE  | EXACT_SIZE;
    [RegisterX64::rax, Size::Auto, OperandType::Immediate, Size::Auto]                                                                          ,  [0x0D              ], X, AUTO_SIZE;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Auto]                                                                       ,  [0x81              ], 1, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                       ,  [0x83              ], 1, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x09              ], X, AUTO_SIZE | LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Auto]                                                                      ,  [0x81              ], 1, AUTO_SIZE ;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x09              ], X, AUTO_SIZE | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x0B              ], X, AUTO_SIZE;
]
"orpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x56        ], X, PREF_66, SSE2;
]
"orps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x56        ], X, DEFAULT, SSE;
]
"outsb" = [
    []                                                                                                                                          ,  [0x6E              ], X, REP;
]
"outsd" = [
    []                                                                                                                                          ,  [0x6F              ], X, REP;
]
"outsw" = [
    []                                                                                                                                          ,  [0x6F              ], X, WORD_SIZE | REP;
]
"pabsb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x1C  ], X, DEFAULT, MMX | SSSE3;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x1C  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x1C  ], X, PREF_66, SSSE3;
]
"pabsd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x1E  ], X, DEFAULT, MMX | SSSE3;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x1E  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x1E  ], X, PREF_66, SSSE3;
]
"pabsw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x1D  ], X, DEFAULT, SSSE3 | MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x1D  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x1D  ], X, PREF_66, SSSE3;
]
"packssdw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x6B        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x6B        ], X, PREF_66, SSE2;
]
"packsswb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x63        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x63        ], X, PREF_66, SSE2;
]
"packusdw" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x2B  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x2B  ], X, PREF_66, SSE41;
]
"packuswb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x67        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x67        ], X, PREF_66, SSE2;
]
"paddb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xFC        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xFC        ], X, PREF_66, SSE2;
]
"paddd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xFE        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xFE        ], X, PREF_66, SSE2;
]
"paddq" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xD4        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xD4        ], X, PREF_66, SSE2;
]
"paddsb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xEC        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xEC        ], X, PREF_66, SSE2;
]
"paddsiw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x51        ], X, DEFAULT, MMX | CYRIX;
]
"paddsw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xED        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xED        ], X, PREF_66, SSE2;
]
"paddusb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xDC        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xDC        ], X, PREF_66, SSE2;
]
"paddusw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xDD        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xDD        ], X, PREF_66, SSE2;
]
"paddw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xFD        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xFD        ], X, PREF_66, SSE2;
]
"palignr" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord, OperandType::Immediate, Size::Byte]                                   ,  [0x0F, 0x3A, 0x0F  ], X, DEFAULT, SSSE3 | MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x0F  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x0F  ], X, PREF_66, SSSE3;
]
"pand" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xDB        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xDB        ], X, PREF_66, SSE2;
]
"pandn" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xDF        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xDF        ], X, PREF_66, SSE2;
]
"pause" = [
    []                                                                                                                                          ,  [0x90              ], X, PREF_F3;
]
"paveb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x50        ], X, DEFAULT, MMX | CYRIX;
]
"pavgb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xE0        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xE0        ], X, PREF_66, SSE2;
]
"pavgusb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0xBF  ], X, IMM_OP, TDNOW;
]
"pavgw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xE3        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xE3        ], X, PREF_66, SSE2;
]
"pblendvb" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x10  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x10  ], X, PREF_66, SSE41;
]
"pblendw" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x0E  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x0E  ], X, PREF_66, SSE41;
]
"pclmulhqhqdq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x3A, 0x44, 0x11], X, IMM_OP | PREF_66, SSE;
]
"pclmulhqlqdq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x3A, 0x44, 0x01], X, PREF_66 | IMM_OP, SSE;
]
"pclmullqhqdq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x3A, 0x44, 0x10], X, PREF_66 | IMM_OP, SSE;
]
"pclmullqlqdq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x3A, 0x44, 0x00], X, PREF_66 | IMM_OP, SSE;
]
"pclmulqdq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x0F, 0x3A, 0x44  ], X, PREF_66, SSE;
]
"pcmpeqb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x74        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x74        ], X, PREF_66, SSE2;
]
"pcmpeqd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x76        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x76        ], X, PREF_66, SSE2;
]
"pcmpeqq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x29  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x29  ], X, PREF_66, SSE41;
]
"pcmpeqw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x75        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x75        ], X, PREF_66, SSE2;
]
"pcmpestri" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x61  ], X, PREF_66, SSE42;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x61  ], X, PREF_66, SSE42;
]
"pcmpestrm" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x60  ], X, PREF_66, SSE42;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x60  ], X, PREF_66, SSE42;
]
"pcmpgtb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x64        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x64        ], X, PREF_66, SSE2;
]
"pcmpgtd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x66        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x66        ], X, PREF_66, SSE2;
]
"pcmpgtq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x37  ], X, PREF_66, SSE42;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x37  ], X, PREF_66, SSE42;
]
"pcmpgtw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x65        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x65        ], X, PREF_66, SSE2;
]
"pcmpistri" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x63  ], X, PREF_66, SSE42;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x63  ], X, PREF_66, SSE42;
]
"pcmpistrm" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x62  ], X, PREF_66, SSE42;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x62  ], X, PREF_66, SSE42;
]
"pdep" = [
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                 ,  [0x02, 0xF5        ], X, VEX_OP | AUTO_REXW | PREF_F2, BMI2;
]
"pdistib" = [
    [RegisterType::MMX, Size::QWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x54        ], X, DEFAULT, MMX | CYRIX;
]
"pext" = [
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                 ,  [0x02, 0xF5        ], X, VEX_OP | AUTO_REXW | PREF_F3, BMI2;
]
"pextrb" = [
    [OperandType::Memory, Size::Byte, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                       ,  [0x0F, 0x3A, 0x14  ], X, ENC_MR | PREF_66, SSE41;
    [RegisterType::Legacy, Size::DWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                     ,  [0x0F, 0x3A, 0x14  ], X, ENC_MR | PREF_66, SSE41;
    [RegisterType::Legacy, Size::QWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                     ,  [0x0F, 0x3A, 0x14  ], X, WITH_REXW | ENC_MR | PREF_66, SSE41;
]
"pextrd" = [
    [OperandType::LegacyMemory, Size::DWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                ,  [0x0F, 0x3A, 0x16  ], X, ENC_MR | PREF_66, SSE41;
]
"pextrq" = [
    [OperandType::LegacyMemory, Size::QWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                ,  [0x0F, 0x3A, 0x16  ], X, WITH_REXW | ENC_MR | PREF_66, SSE41;
]
"pextrw" = [
    [OperandType::Memory, Size::Word, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                       ,  [0x0F, 0x3A, 0x15  ], X, ENC_MR | PREF_66, SSE41;
    [RegisterType::Legacy, Size::DWord, RegisterType::MMX, Size::QWord, OperandType::Immediate, Size::Byte]                                     ,  [0x0F, 0xC5        ], X, DEFAULT, MMX;
    [RegisterType::Legacy, Size::DWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                     ,  [0x0F, 0xC5        ], X, PREF_66, SSE2;
    [RegisterType::Legacy, Size::DWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                     ,  [0x0F, 0x3A, 0x15  ], X, ENC_MR | PREF_66, SSE41;
    [RegisterType::Legacy, Size::QWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                     ,  [0x0F, 0x3A, 0x15  ], X, WITH_REXW | ENC_MR | PREF_66, SSE41;
]
"pf2id" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x1D  ], X, IMM_OP, TDNOW;
]
"pf2iw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x1C  ], X, IMM_OP, TDNOW;
]
"pfacc" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0xAE  ], X, IMM_OP, TDNOW;
]
"pfadd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x9E  ], X, IMM_OP, TDNOW;
]
"pfcmpeq" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0xB0  ], X, IMM_OP, TDNOW;
]
"pfcmpge" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x90  ], X, IMM_OP, TDNOW;
]
"pfcmpgt" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0xA0  ], X, IMM_OP, TDNOW;
]
"pfmax" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0xA4  ], X, IMM_OP, TDNOW;
]
"pfmin" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x94  ], X, IMM_OP, TDNOW;
]
"pfmul" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0xB4  ], X, IMM_OP, TDNOW;
]
"pfnacc" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x8A  ], X, IMM_OP, TDNOW;
]
"pfpnacc" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x8E  ], X, IMM_OP, TDNOW;
]
"pfrcp" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x96  ], X, IMM_OP, TDNOW;
]
"pfrcpit1" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0xA6  ], X, IMM_OP, TDNOW;
]
"pfrcpit2" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0xB6  ], X, IMM_OP, TDNOW;
]
"pfrcpv" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x86  ], X, IMM_OP, TDNOW | CYRIX;
]
"pfrsqit1" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0xA7  ], X, IMM_OP, TDNOW;
]
"pfrsqrt" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x97  ], X, IMM_OP, TDNOW;
]
"pfrsqrtv" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x87  ], X, IMM_OP, CYRIX | TDNOW;
]
"pfsub" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x9A  ], X, IMM_OP, TDNOW;
]
"pfsubr" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0xAA  ], X, IMM_OP, TDNOW;
]
"phaddd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x02  ], X, DEFAULT, MMX | SSSE3;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x02  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x02  ], X, PREF_66, SSSE3;
]
"phaddsw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x03  ], X, DEFAULT, MMX | SSSE3;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x03  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x03  ], X, PREF_66, SSSE3;
]
"phaddw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x01  ], X, DEFAULT, MMX | SSSE3;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x01  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x01  ], X, PREF_66, SSSE3;
]
"phminposuw" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x41  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x41  ], X, PREF_66, SSE41;
]
"phsubd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x06  ], X, DEFAULT, SSSE3 | MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x06  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x06  ], X, PREF_66, SSSE3;
]
"phsubsw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x07  ], X, DEFAULT, SSSE3 | MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x07  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x07  ], X, PREF_66, SSSE3;
]
"phsubw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x05  ], X, DEFAULT, SSSE3 | MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x05  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x05  ], X, PREF_66, SSSE3;
]
"pi2fd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x0D  ], X, IMM_OP, TDNOW;
]
"pi2fw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0x0C  ], X, IMM_OP, TDNOW;
]
"pinsrb" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::MemAuto, OperandType::Immediate, Size::Byte]                                    ,  [0x0F, 0x3A, 0x20  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::Legacy, Size::DWord, OperandType::Immediate, Size::Byte]                                     ,  [0x0F, 0x3A, 0x20  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::Byte, OperandType::Immediate, Size::Byte]                                 ,  [0x0F, 0x3A, 0x20  ], X, PREF_66, SSE41;
]
"pinsrd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::MemAuto, OperandType::Immediate, Size::Byte]                                    ,  [0x0F, 0x3A, 0x22  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::DWord, OperandType::Immediate, Size::Byte]                                ,  [0x0F, 0x3A, 0x22  ], X, PREF_66, SSE41;
]
"pinsrq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::MemAuto, OperandType::Immediate, Size::Byte]                                    ,  [0x0F, 0x3A, 0x22  ], X, WITH_REXW | PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::QWord, OperandType::Immediate, Size::Byte]                                ,  [0x0F, 0x3A, 0x22  ], X, WITH_REXW | PREF_66, SSE41;
]
"pinsrw" = [
    [RegisterType::MMX, Size::QWord, OperandType::Memory, Size::MemAuto, OperandType::Immediate, Size::Byte]                                    ,  [0x0F, 0xC4        ], X, DEFAULT, MMX;
    [RegisterType::MMX, Size::QWord, RegisterType::Legacy, Size::DWord, OperandType::Immediate, Size::Byte]                                     ,  [0x0F, 0xC4        ], X, DEFAULT, MMX;
    [RegisterType::MMX, Size::QWord, OperandType::LegacyMemory, Size::Word, OperandType::Immediate, Size::Byte]                                 ,  [0x0F, 0xC4        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::MemAuto, OperandType::Immediate, Size::Byte]                                    ,  [0x0F, 0xC4        ], X, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::Word, OperandType::Immediate, Size::Byte]                                       ,  [0x0F, 0xC4        ], X, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::Legacy, Size::DWord, OperandType::Immediate, Size::Byte]                                     ,  [0x0F, 0xC4        ], X, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::Legacy, Size::Word, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0xC4        ], X, PREF_66, SSE2;
]
"pmachriw" = [
    [RegisterType::MMX, Size::QWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x5E        ], X, DEFAULT, MMX | CYRIX;
]
"pmaddubsw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x04  ], X, DEFAULT, MMX | SSSE3;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x04  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x04  ], X, PREF_66, SSSE3;
]
"pmaddwd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xF5        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xF5        ], X, PREF_66, SSE2;
]
"pmagw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x52        ], X, DEFAULT, CYRIX | MMX;
]
"pmaxsb" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x3C  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x3C  ], X, PREF_66, SSE41;
]
"pmaxsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x3D  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x3D  ], X, PREF_66, SSE41;
]
"pmaxsw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xEE        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xEE        ], X, PREF_66, SSE2;
]
"pmaxub" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xDE        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xDE        ], X, PREF_66, SSE2;
]
"pmaxud" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x3F  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x3F  ], X, PREF_66, SSE41;
]
"pmaxuw" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x3E  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x3E  ], X, PREF_66, SSE41;
]
"pminsb" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x38  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x38  ], X, PREF_66, SSE41;
]
"pminsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x39  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x39  ], X, PREF_66, SSE41;
]
"pminsw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xEA        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xEA        ], X, PREF_66, SSE2;
]
"pminub" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xDA        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xDA        ], X, PREF_66, SSE2;
]
"pminud" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x3B  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x3B  ], X, PREF_66, SSE41;
]
"pminuw" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x3A  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x3A  ], X, PREF_66, SSE41;
]
"pmovmskb" = [
    [RegisterType::Legacy, Size::DWord, RegisterType::MMX, Size::QWord]                                                                         ,  [0x0F, 0xD7        ], X, DEFAULT, MMX;
    [RegisterType::Legacy, Size::DWord, RegisterType::AVX, Size::OWord]                                                                         ,  [0x0F, 0xD7        ], X, PREF_66, SSE2;
]
"pmovsxbd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x38, 0x21  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x21  ], X, PREF_66, SSE41;
]
"pmovsxbq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::Word]                                                                           ,  [0x0F, 0x38, 0x22  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x22  ], X, PREF_66, SSE41;
]
"pmovsxbw" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x20  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x20  ], X, PREF_66, SSE41;
]
"pmovsxdq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x25  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x25  ], X, PREF_66, SSE41;
]
"pmovsxwd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x23  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x23  ], X, PREF_66, SSE41;
]
"pmovsxwq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x38, 0x24  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x24  ], X, PREF_66, SSE41;
]
"pmovzxbd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x38, 0x31  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x31  ], X, PREF_66, SSE41;
]
"pmovzxbq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::Word]                                                                           ,  [0x0F, 0x38, 0x32  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x32  ], X, PREF_66, SSE41;
]
"pmovzxbw" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x30  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x30  ], X, PREF_66, SSE41;
]
"pmovzxdq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x35  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x35  ], X, PREF_66, SSE41;
]
"pmovzxwd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x33  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x33  ], X, PREF_66, SSE41;
]
"pmovzxwq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x38, 0x34  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x34  ], X, PREF_66, SSE41;
]
"pmuldq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x28  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x28  ], X, PREF_66, SSE41;
]
"pmulhriw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x5D        ], X, DEFAULT, CYRIX | MMX;
]
"pmulhrsw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x0B  ], X, DEFAULT, MMX | SSSE3;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x0B  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x0B  ], X, PREF_66, SSSE3;
]
"pmulhrwa" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0xB7  ], X, IMM_OP, TDNOW;
]
"pmulhrwc" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x59        ], X, DEFAULT, MMX | CYRIX;
]
"pmulhuw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xE4        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xE4        ], X, PREF_66, SSE2;
]
"pmulhw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xE5        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xE5        ], X, PREF_66, SSE2;
]
"pmulld" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x40  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x40  ], X, PREF_66, SSE41;
]
"pmullw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xD5        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xD5        ], X, PREF_66, SSE2;
]
"pmuludq" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xF4        ], X, DEFAULT, SSE2;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xF4        ], X, PREF_66, SSE2;
]
"pmvgezb" = [
    [RegisterType::MMX, Size::QWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x5C        ], X, DEFAULT, CYRIX | MMX;
]
"pmvlzb" = [
    [RegisterType::MMX, Size::QWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x5B        ], X, DEFAULT, CYRIX | MMX;
]
"pmvnzb" = [
    [RegisterType::MMX, Size::QWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x5A        ], X, DEFAULT, CYRIX | MMX;
]
"pmvzb" = [
    [RegisterType::MMX, Size::QWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x58        ], X, DEFAULT, MMX | CYRIX;
]
"pop" = [
    [RegisterX64::es, Size::Word]                                                                                                               ,  [0x07              ], X, X86_ONLY;
    [RegisterX64::ss, Size::Word]                                                                                                               ,  [0x17              ], X, X86_ONLY;
    [RegisterX64::ds, Size::Word]                                                                                                               ,  [0x1F              ], X, X86_ONLY;
    [RegisterX64::fs, Size::Word]                                                                                                               ,  [0x0F, 0xA1        ], X;
    [RegisterX64::gs, Size::Word]                                                                                                               ,  [0x0F, 0xA9        ], X;
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0x58              ], X, AUTO_NO32 | SHORT_ARG;
    [OperandType::LegacyMemory, Size::Auto]                                                                                                     ,  [0x8F              ], 0, AUTO_NO32;
]
"popa" = [
    []                                                                                                                                          ,  [0x61              ], X, X86_ONLY | WORD_SIZE;
]
"popad" = [
    []                                                                                                                                          ,  [0x61              ], X, X86_ONLY;
]
"popcnt" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x0F, 0xB8        ], X, AUTO_SIZE | PREF_F3;
]
"popf" = [
    []                                                                                                                                          ,  [0x9D              ], X;
]
"popfq" = [
    []                                                                                                                                          ,  [0x9D              ], X;
]
"popfw" = [
    []                                                                                                                                          ,  [0x9D              ], X, WORD_SIZE;
]
"por" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xEB        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xEB        ], X, PREF_66, SSE2;
]
"prefetch" = [
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0x0F, 0x0D        ], 0, DEFAULT, TDNOW;
]
"prefetchnta" = [
    [OperandType::Memory, Size::Byte]                                                                                                           ,  [0x0F, 0x18        ], 0;
]
"prefetcht0" = [
    [OperandType::Memory, Size::Byte]                                                                                                           ,  [0x0F, 0x18        ], 1;
]
"prefetcht1" = [
    [OperandType::Memory, Size::Byte]                                                                                                           ,  [0x0F, 0x18        ], 2;
]
"prefetcht2" = [
    [OperandType::Memory, Size::Byte]                                                                                                           ,  [0x0F, 0x18        ], 3;
]
"prefetchw" = [
    [OperandType::Memory, Size::QWord]                                                                                                          ,  [0x0F, 0x0D        ], 1, DEFAULT, TDNOW;
]
"prefetchwt1" = [
    [OperandType::Memory, Size::Byte]                                                                                                           ,  [0x0F, 0x0D        ], 2, DEFAULT, PREFETCHWT1;
]
"psadbw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xF6        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xF6        ], X, PREF_66, SSE2;
]
"pshufb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x00  ], X, DEFAULT, SSSE3 | MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x00  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x00  ], X, PREF_66, SSSE3;
]
"pshufd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x0F, 0x70        ], X, PREF_66, SSE2;
]
"pshufhw" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x0F, 0x70        ], X, PREF_F3, SSE2;
]
"pshuflw" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x0F, 0x70        ], X, PREF_F2, SSE2;
]
"pshufw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord, OperandType::Immediate, Size::Byte]                                   ,  [0x0F, 0x70        ], X, DEFAULT, MMX;
]
"psignb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x08  ], X, DEFAULT, MMX | SSSE3;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x08  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x08  ], X, PREF_66, SSSE3;
]
"psignd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x0A  ], X, DEFAULT, SSSE3 | MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x0A  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x0A  ], X, PREF_66, SSSE3;
]
"psignw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x38, 0x09  ], X, DEFAULT, MMX | SSSE3;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x09  ], X, PREF_66, SSSE3;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x09  ], X, PREF_66, SSSE3;
]
"pslld" = [
    [RegisterType::MMX, Size::QWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x72        ], 6, DEFAULT, MMX;
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xF2        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x72        ], 6, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xF2        ], X, PREF_66, SSE2;
]
"pslldq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x73        ], 7, PREF_66, SSE2;
]
"psllq" = [
    [RegisterType::MMX, Size::QWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x73        ], 6, DEFAULT, MMX;
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xF3        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x73        ], 6, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xF3        ], X, PREF_66, SSE2;
]
"psllw" = [
    [RegisterType::MMX, Size::QWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x71        ], 6, DEFAULT, MMX;
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xF1        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x71        ], 6, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xF1        ], X, PREF_66, SSE2;
]
"psrad" = [
    [RegisterType::MMX, Size::QWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x72        ], 4, DEFAULT, MMX;
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xE2        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x72        ], 4, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xE2        ], X, PREF_66, SSE2;
]
"psraw" = [
    [RegisterType::MMX, Size::QWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x71        ], 4, DEFAULT, MMX;
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xE1        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x71        ], 4, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xE1        ], X, PREF_66, SSE2;
]
"psrld" = [
    [RegisterType::MMX, Size::QWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x72        ], 2, DEFAULT, MMX;
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xD2        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x72        ], 2, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xD2        ], X, PREF_66, SSE2;
]
"psrldq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x73        ], 3, PREF_66, SSE2;
]
"psrlq" = [
    [RegisterType::MMX, Size::QWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x73        ], 2, DEFAULT, MMX;
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xD3        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x73        ], 2, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xD3        ], X, PREF_66, SSE2;
]
"psrlw" = [
    [RegisterType::MMX, Size::QWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x71        ], 2, DEFAULT, MMX;
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xD1        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                                                        ,  [0x0F, 0x71        ], 2, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xD1        ], X, PREF_66, SSE2;
]
"psubb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xF8        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xF8        ], X, PREF_66, SSE2;
]
"psubd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xFA        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xFA        ], X, PREF_66, SSE2;
]
"psubq" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xFB        ], X, DEFAULT, SSE2;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xFB        ], X, PREF_66, SSE2;
]
"psubsb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xE8        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xE8        ], X, PREF_66, SSE2;
]
"psubsiw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x55        ], X, DEFAULT, CYRIX | MMX;
]
"psubsw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xE9        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xE9        ], X, PREF_66, SSE2;
]
"psubusb" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xD8        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xD8        ], X, PREF_66, SSE2;
]
"psubusw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xD9        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xD9        ], X, PREF_66, SSE2;
]
"psubw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xF9        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xF9        ], X, PREF_66, SSE2;
]
"pswapd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x0F, 0xBB  ], X, IMM_OP, TDNOW;
]
"ptest" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x38, 0x17  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x38, 0x17  ], X, PREF_66, SSE41;
]
"punpckhbw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x68        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x68        ], X, PREF_66, SSE2;
]
"punpckhdq" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x6A        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x6A        ], X, PREF_66, SSE2;
]
"punpckhqdq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x6D        ], X, PREF_66, SSE2;
]
"punpckhwd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x69        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x69        ], X, PREF_66, SSE2;
]
"punpcklbw" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x60        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x60        ], X, PREF_66, SSE2;
]
"punpckldq" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x62        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x62        ], X, PREF_66, SSE2;
]
"punpcklqdq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x6C        ], X, PREF_66, SSE2;
]
"punpcklwd" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0x61        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x61        ], X, PREF_66, SSE2;
]
"push" = [
    [RegisterX64::es, Size::Word]                                                                                                               ,  [0x06              ], X, X86_ONLY;
    [RegisterX64::cs, Size::Word]                                                                                                               ,  [0x0E              ], X, X86_ONLY;
    [RegisterX64::ss, Size::Word]                                                                                                               ,  [0x16              ], X, X86_ONLY;
    [RegisterX64::ds, Size::Word]                                                                                                               ,  [0x1E              ], X, X86_ONLY;
    [RegisterX64::fs, Size::Word]                                                                                                               ,  [0x0F, 0xA0        ], X;
    [RegisterX64::gs, Size::Word]                                                                                                               ,  [0x0F, 0xA8        ], X;
    [OperandType::Immediate, Size::Byte]                                                                                                        ,  [0x6A              ], X, EXACT_SIZE;
    [OperandType::Immediate, Size::Word]                                                                                                        ,  [0x68              ], X, EXACT_SIZE | WORD_SIZE;
    [OperandType::Immediate, Size::DWord]                                                                                                       ,  [0x68              ], X;
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0x50              ], X, AUTO_NO32 | SHORT_ARG;
    [OperandType::LegacyMemory, Size::Auto]                                                                                                     ,  [0xFF              ], 6, AUTO_NO32;
]
"pusha" = [
    []                                                                                                                                          ,  [0x60              ], X, X86_ONLY | WORD_SIZE;
]
"pushad" = [
    []                                                                                                                                          ,  [0x60              ], X, X86_ONLY;
]
"pushf" = [
    []                                                                                                                                          ,  [0x9C              ], X;
]
"pushfq" = [
    []                                                                                                                                          ,  [0x9C              ], X;
]
"pushfw" = [
    []                                                                                                                                          ,  [0x9C              ], X, WORD_SIZE;
]
"pxor" = [
    [RegisterType::MMX, Size::QWord, OperandType::MMXMemory, Size::QWord]                                                                       ,  [0x0F, 0xEF        ], X, DEFAULT, MMX;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0xEF        ], X, PREF_66, SSE2;
]
"rcl" = [
    [OperandType::LegacyMemory, Size::Byte, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD2              ], 2;
    [OperandType::LegacyMemory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC0              ], 2;
    [OperandType::LegacyMemory, Size::Auto, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD3              ], 2, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC1              ], 2, AUTO_SIZE;
]
"rcpps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x53        ], X, DEFAULT, SSE;
]
"rcpss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x53        ], X, PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x53        ], X, PREF_F3, SSE;
]
"rcr" = [
    [OperandType::LegacyMemory, Size::Byte, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD2              ], 3;
    [OperandType::LegacyMemory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC0              ], 3;
    [OperandType::LegacyMemory, Size::Auto, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD3              ], 3, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC1              ], 3, AUTO_SIZE;
]
"rdfsbase" = [
    [RegisterType::Legacy, Size::DWord]                                                                                                         ,  [0x0F, 0xAE        ], 0, PREF_F3;
    [RegisterType::Legacy, Size::QWord]                                                                                                         ,  [0x0F, 0xAE        ], 0, WITH_REXW | PREF_F3;
]
"rdgsbase" = [
    [RegisterType::Legacy, Size::DWord]                                                                                                         ,  [0x0F, 0xAE        ], 1, PREF_F3;
    [RegisterType::Legacy, Size::QWord]                                                                                                         ,  [0x0F, 0xAE        ], 1, WITH_REXW | PREF_F3;
]
"rdm" = [
    []                                                                                                                                          ,  [0x0F, 0x3A        ], X, DEFAULT, CYRIX;
]
"rdmsr" = [
    []                                                                                                                                          ,  [0x0F, 0x32        ], X;
]
"rdpid" = [
    [RegisterType::Legacy, Size::QWord]                                                                                                         ,  [0x0F, 0xC7        ], 7, PREF_F3;
]
"rdpkru" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xEE  ], X;
]
"rdpmc" = [
    []                                                                                                                                          ,  [0x0F, 0x33        ], X;
]
"rdrand" = [
    [RegisterType::Legacy, Size::QWord]                                                                                                         ,  [0x0F, 0xC7        ], 6, WITH_REXW;
]
"rdseed" = [
    [RegisterType::Legacy, Size::QWord]                                                                                                         ,  [0x0F, 0xC7        ], 7, WITH_REXW;
]
"rdshr" = [
    [OperandType::LegacyMemory, Size::DWord]                                                                                                    ,  [0x0F, 0x36        ], 0, DEFAULT, CYRIX;
]
"rdtsc" = [
    []                                                                                                                                          ,  [0x0F, 0x31        ], X;
]
"rdtscp" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xF9  ], X;
]
"ret" = [
    []                                                                                                                                          ,  [0xC3              ], X;
    [OperandType::Immediate, Size::Word]                                                                                                        ,  [0xC2              ], X;
]
"retf" = [
    []                                                                                                                                          ,  [0xCB              ], X;
    [OperandType::Immediate, Size::Word]                                                                                                        ,  [0xCA              ], X;
]
"retn" = [
    []                                                                                                                                          ,  [0xC3              ], X;
    [OperandType::Immediate, Size::Word]                                                                                                        ,  [0xC2              ], X;
]
"rol" = [
    [OperandType::LegacyMemory, Size::Byte, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD2              ], 0;
    [OperandType::LegacyMemory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC0              ], 0;
    [OperandType::LegacyMemory, Size::Auto, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD3              ], 0, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC1              ], 0, AUTO_SIZE;
]
"ror" = [
    [OperandType::LegacyMemory, Size::Byte, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD2              ], 1;
    [OperandType::LegacyMemory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC0              ], 1;
    [OperandType::LegacyMemory, Size::Auto, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD3              ], 1, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC1              ], 1, AUTO_SIZE;
]
"rorx" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Byte]                               ,  [0x03, 0xF0        ], X, VEX_OP | AUTO_REXW | PREF_F2, BMI2;
]
"roundpd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x09  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x09  ], X, PREF_66, SSE41;
]
"roundps" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x08  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x08  ], X, PREF_66, SSE41;
]
"roundsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x0B  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x0B  ], X, PREF_66, SSE41;
]
"roundss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]                                      ,  [0x0F, 0x3A, 0x0A  ], X, PREF_66, SSE41;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                        ,  [0x0F, 0x3A, 0x0A  ], X, PREF_66, SSE41;
]
"rsdc" = [
    [RegisterType::Segment, Size::Word, OperandType::Memory, Size::PWord]                                                                       ,  [0x0F, 0x79        ], X, EXACT_SIZE, CYRIX;
]
"rsldt" = [
    [OperandType::Memory, Size::PWord]                                                                                                          ,  [0x0F, 0x7B        ], 0, EXACT_SIZE, CYRIX;
]
"rsm" = [
    []                                                                                                                                          ,  [0x0F, 0xAA        ], X;
]
"rsqrtps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x52        ], X, DEFAULT, SSE;
]
"rsqrtss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x52        ], X, PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x52        ], X, PREF_F3, SSE;
]
"rsts" = [
    [OperandType::Memory, Size::PWord]                                                                                                          ,  [0x0F, 0x7D        ], 0, EXACT_SIZE, CYRIX;
]
"sahf" = [
    []                                                                                                                                          ,  [0x9E              ], X;
]
"sal" = [
    [OperandType::LegacyMemory, Size::Byte, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD2              ], 4;
    [OperandType::LegacyMemory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC0              ], 4;
    [OperandType::LegacyMemory, Size::Auto, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD3              ], 4, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC1              ], 4, AUTO_SIZE;
]
"sar" = [
    [OperandType::LegacyMemory, Size::Byte, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD2              ], 7;
    [OperandType::LegacyMemory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC0              ], 7;
    [OperandType::LegacyMemory, Size::Auto, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD3              ], 7, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC1              ], 7, AUTO_SIZE;
]
"sarx" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto, RegisterType::Legacy, Size::Auto]                                 ,  [0x02, 0xF7        ], X, VEX_OP | AUTO_REXW | ENC_MR | PREF_F3, BMI2;
]
"sbb" = [
    [RegisterX64::rax, Size::Byte, OperandType::Immediate, Size::Byte]                                                                          ,  [0x1C              ], X;
    [OperandType::Memory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                       ,  [0x80              ], 3, LOCK;
    [OperandType::Memory, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                         ,  [0x18              ], X, LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::Immediate, Size::Byte]                                                                      ,  [0x80              ], 3;
    [RegisterType::Legacy, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                        ,  [0x18              ], X, ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::LegacyMemory, Size::Byte]                                                                   ,  [0x1A              ], X;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Byte]                                                                      ,  [0x83              ], 3, AUTO_SIZE  | EXACT_SIZE;
    [RegisterX64::rax, Size::Auto, OperandType::Immediate, Size::Auto]                                                                          ,  [0x1D              ], X, AUTO_SIZE;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Auto]                                                                       ,  [0x81              ], 3, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                       ,  [0x83              ], 3, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x19              ], X, AUTO_SIZE | LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Auto]                                                                      ,  [0x81              ], 3, AUTO_SIZE ;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x19              ], X, AUTO_SIZE | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x1B              ], X, AUTO_SIZE;
]
"scasb" = [
    []                                                                                                                                          ,  [0xAE              ], X, REPE;
]
"scasd" = [
    []                                                                                                                                          ,  [0xAF              ], X, REPE;
]
"scasq" = [
    []                                                                                                                                          ,  [0xAF              ], X, REPE | WITH_REXW;
]
"scasw" = [
    []                                                                                                                                          ,  [0xAF              ], X, REPE | WORD_SIZE;
]
"sfence" = [
    []                                                                                                                                          ,  [0x0F, 0xAE, 0xF8  ], X, DEFAULT, AMD;
]
"sgdt" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0x01        ], 0;
]
"sha1msg1" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x38, 0xC9  ], X, DEFAULT, SHA;
]
"sha1msg2" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x38, 0xCA  ], X, DEFAULT, SHA;
]
"sha1nexte" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x38, 0xC8  ], X, DEFAULT, SHA;
]
"sha1rnds4" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x0F, 0x3A, 0xCC  ], X, DEFAULT, SHA;
]
"sha256msg1" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x38, 0xCC  ], X, DEFAULT, SHA;
]
"sha256msg2" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x38, 0xCD  ], X, DEFAULT, SHA;
]
"sha256rnds2" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x38, 0xCB  ], X, DEFAULT, SHA;
]
"shl" = [
    [OperandType::LegacyMemory, Size::Byte, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD2              ], 4;
    [OperandType::LegacyMemory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC0              ], 4;
    [OperandType::LegacyMemory, Size::Auto, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD3              ], 4, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC1              ], 4, AUTO_SIZE;
]
"shld" = [
    [OperandType::LegacyMemory, Size::Auto, RegisterType::Legacy, Size::Auto, RegisterX64::rcx, Size::Byte]                                     ,  [0x0F, 0xA5        ], X, AUTO_SIZE | ENC_MR;
    [OperandType::LegacyMemory, Size::Auto, RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Byte]                               ,  [0x0F, 0xA4        ], X, AUTO_SIZE | ENC_MR;
]
"shlx" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto, RegisterType::Legacy, Size::Auto]                                 ,  [0x02, 0xF7        ], X, VEX_OP | AUTO_REXW | ENC_MR | PREF_66, BMI2;
]
"shr" = [
    [OperandType::LegacyMemory, Size::Byte, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD2              ], 5;
    [OperandType::LegacyMemory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC0              ], 5;
    [OperandType::LegacyMemory, Size::Auto, RegisterX64::rcx, Size::Byte]                                                                       ,  [0xD3              ], 5, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC1              ], 5, AUTO_SIZE;
]
"shrd" = [
    [OperandType::LegacyMemory, Size::Auto, RegisterType::Legacy, Size::Auto, RegisterX64::rcx, Size::Byte]                                     ,  [0x0F, 0xAD        ], X, AUTO_SIZE | ENC_MR;
    [OperandType::LegacyMemory, Size::Auto, RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Byte]                               ,  [0x0F, 0xAC        ], X, AUTO_SIZE | ENC_MR;
]
"shrx" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto, RegisterType::Legacy, Size::Auto]                                 ,  [0x02, 0xF7        ], X, VEX_OP | AUTO_REXW | ENC_MR | PREF_F2, BMI2;
]
"shufpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x0F, 0xC6        ], X, PREF_66, SSE2;
]
"shufps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x0F, 0xC6        ], X, DEFAULT, SSE;
]
"sidt" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0x01        ], 1;
]
"skinit" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xDE  ], X;
]
"sldt" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0x00        ], 0;
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0x0F, 0x00        ], 0, AUTO_SIZE;
]
"slwpcb" = [
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0x09, 0x12        ], 1, XOP_OP | AUTO_REXW, AMD;
]
"smint" = [
    []                                                                                                                                          ,  [0x0F, 0x38        ], X, DEFAULT, CYRIX;
]
"smsw" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0x01        ], 4;
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0x0F, 0x01        ], 4, AUTO_SIZE;
]
"sqrtpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x51        ], X, PREF_66, SSE2;
]
"sqrtps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x51        ], X, DEFAULT, SSE;
]
"sqrtsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x51        ], X, PREF_F2, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x51        ], X, PREF_F2, SSE2;
]
"sqrtss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x51        ], X, PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x51        ], X, PREF_F3, SSE;
]
"stac" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xCB  ], X;
]
"stc" = [
    []                                                                                                                                          ,  [0xF9              ], X;
]
"std" = [
    []                                                                                                                                          ,  [0xFD              ], X;
]
"stgi" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xDC  ], X, DEFAULT, VMX | AMD;
]
"sti" = [
    []                                                                                                                                          ,  [0xFB              ], X;
]
"stmxcsr" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0x0F, 0xAE        ], 3, DEFAULT, SSE;
]
"stosb" = [
    []                                                                                                                                          ,  [0xAA              ], X, REP;
]
"stosd" = [
    []                                                                                                                                          ,  [0xAB              ], X, REP;
]
"stosq" = [
    []                                                                                                                                          ,  [0xAB              ], X, WITH_REXW | REP;
]
"stosw" = [
    []                                                                                                                                          ,  [0xAB              ], X, WORD_SIZE | REP;
]
"str" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0x00        ], 1;
    [RegisterType::Legacy, Size::Auto]                                                                                                          ,  [0x0F, 0x00        ], 1, AUTO_SIZE;
]
"sub" = [
    [RegisterX64::rax, Size::Byte, OperandType::Immediate, Size::Byte]                                                                          ,  [0x2C              ], X;
    [OperandType::Memory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                       ,  [0x80              ], 5, LOCK;
    [OperandType::Memory, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                         ,  [0x28              ], X, LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::Immediate, Size::Byte]                                                                      ,  [0x80              ], 5;
    [RegisterType::Legacy, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                        ,  [0x28              ], X, ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::LegacyMemory, Size::Byte]                                                                   ,  [0x2A              ], X;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Byte]                                                                      ,  [0x83              ], 5, AUTO_SIZE  | EXACT_SIZE;
    [RegisterX64::rax, Size::Auto, OperandType::Immediate, Size::Auto]                                                                          ,  [0x2D              ], X, AUTO_SIZE;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Auto]                                                                       ,  [0x81              ], 5, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                       ,  [0x83              ], 5, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x29              ], X, AUTO_SIZE | LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Auto]                                                                      ,  [0x81              ], 5, AUTO_SIZE ;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x29              ], X, AUTO_SIZE | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x2B              ], X, AUTO_SIZE;
]
"subpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x5C        ], X, PREF_66, SSE2;
]
"subps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x5C        ], X, DEFAULT, SSE;
]
"subsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x5C        ], X, PREF_F2, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x5C        ], X, PREF_F2, SSE2;
]
"subss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x5C        ], X, PREF_F3, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x5C        ], X, PREF_F3, SSE;
]
"svdc" = [
    [OperandType::Memory, Size::PWord, RegisterType::Segment, Size::Word]                                                                       ,  [0x0F, 0x78        ], X, ENC_MR | EXACT_SIZE, CYRIX;
]
"svldt" = [
    [OperandType::Memory, Size::PWord]                                                                                                          ,  [0x0F, 0x7A        ], 0, EXACT_SIZE, CYRIX;
]
"svts" = [
    [OperandType::Memory, Size::PWord]                                                                                                          ,  [0x0F, 0x7C        ], 0, EXACT_SIZE, CYRIX;
]
"swapgs" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xF8  ], X;
]
"syscall" = [
    []                                                                                                                                          ,  [0x0F, 0x05        ], X, DEFAULT, AMD;
]
"sysenter" = [
    []                                                                                                                                          ,  [0x0F, 0x34        ], X, X86_ONLY;
]
"sysexit" = [
    []                                                                                                                                          ,  [0x0F, 0x35        ], X, X86_ONLY;
]
"sysret" = [
    []                                                                                                                                          ,  [0x0F, 0x07        ], X, DEFAULT, AMD;
]
"t1mskc" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x09, 0x01        ], 7, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"test" = [
    [RegisterX64::rax, Size::Byte, OperandType::Immediate, Size::Byte]                                                                          ,  [0xA8              ], X;
    [RegisterType::Legacy, Size::Byte, OperandType::Memory, Size::Byte]                                                                         ,  [0x84              ], X;
    [OperandType::LegacyMemory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                 ,  [0xF6              ], 0;
    [OperandType::LegacyMemory, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                   ,  [0x84              ], X, ENC_MR;
    [RegisterX64::rax, Size::Auto, OperandType::Immediate, Size::Auto]                                                                          ,  [0xA9              ], X, AUTO_SIZE;
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::Auto]                                                                         ,  [0x85              ], X, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Auto]                                                                 ,  [0xF7              ], 0, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                   ,  [0x85              ], X, AUTO_SIZE | ENC_MR;
]
"tzcnt" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x0F, 0xBC        ], X, AUTO_SIZE | PREF_F3, BMI1;
]
"tzmsk" = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x09, 0x01        ], 4, XOP_OP | AUTO_REXW | ENC_VM, TBM;
]
"ucomisd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x0F, 0x2E        ], X, PREF_66, SSE2;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x2E        ], X, PREF_66, SSE2;
]
"ucomiss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x0F, 0x2E        ], X, DEFAULT, SSE;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x0F, 0x2E        ], X, DEFAULT, SSE;
]
"ud2" = [
    []                                                                                                                                          ,  [0x0F, 0x0B        ], X;
]
"ud2a" = [
    []                                                                                                                                          ,  [0x0F, 0x0B        ], X;
]
"unpckhpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x15        ], X, PREF_66, SSE2;
]
"unpckhps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x15        ], X, DEFAULT, SSE;
]
"unpcklpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x14        ], X, PREF_66, SSE2;
]
"unpcklps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x14        ], X, DEFAULT, SSE;
]
"vaddpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x58        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vaddps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x58        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vaddsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0x58        ], X, VEX_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x58        ], X, VEX_OP | PREF_F2, AVX;
]
"vaddss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x01, 0x58        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x58        ], X, VEX_OP | PREF_F3, AVX;
]
"vaddsubpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xD0        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vaddsubps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xD0        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
]
"vaesdec" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x02, 0xDE        ], X, VEX_OP | PREF_66, AVX;
]
"vaesdeclast" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x02, 0xDF        ], X, VEX_OP | PREF_66, AVX;
]
"vaesenc" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x02, 0xDC        ], X, VEX_OP | PREF_66, AVX;
]
"vaesenclast" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x02, 0xDD        ], X, VEX_OP | PREF_66, AVX;
]
"vaesimc" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x02, 0xDB        ], X, VEX_OP | PREF_66, AVX;
]
"vaeskeygenassist" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x03, 0xDF        ], X, VEX_OP | PREF_66, AVX;
]
"vandnpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x55        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vandnps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x55        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vandpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x54        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vandps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x54        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vblendpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                  ,  [0x03, 0x0D        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vblendps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                  ,  [0x03, 0x0C        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vblendvpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x4B        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vblendvps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x4A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vbroadcastf128" = [
    [RegisterType::AVX, Size::HWord, OperandType::Memory, Size::OWord]                                                                          ,  [0x02, 0x1A        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX;
]
"vbroadcasti128" = [
    [RegisterType::AVX, Size::HWord, OperandType::Memory, Size::OWord]                                                                          ,  [0x02, 0x5A        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX2;
]
"vbroadcastsd" = [
    [RegisterType::AVX, Size::HWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x02, 0x19        ], X, VEX_OP | WITH_VEXL | PREF_66, AVX;
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x02, 0x19        ], X, VEX_OP | WITH_VEXL | PREF_66, AVX;
]
"vbroadcastss" = [
    [RegisterType::AVX, Size::Auto, OperandType::Memory, Size::DWord]                                                                           ,  [0x02, 0x18        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::OWord]                                                                             ,  [0x02, 0x18        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vcmpeq_ospd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpeq_osps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpeq_ossd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpeq_osss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x10  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpeq_uqpd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x08  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x08  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpeq_uqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x08  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpeq_uqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x08  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x08  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpeq_uqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x08  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x08  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpeq_uspd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x18  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x18  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpeq_usps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x18  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpeq_ussd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x18  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x18  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpeq_usss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x18  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x18  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpeqpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x00  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpeqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x00  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpeqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x00  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x00  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpeqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x00  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x00  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpfalse_oqpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpfalse_oqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpfalse_oqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpfalse_oqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpfalse_ospd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x1B  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpfalse_osps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x1B  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpfalse_ossd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x1B  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x1B  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpfalse_osss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x1B  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x1B  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpfalsepd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x0B  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpfalseps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpfalsesd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpfalsess" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0B  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpge_oqpd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x1D  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x1D  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpge_oqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x1D  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpge_oqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x1D  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x1D  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpge_oqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x1D  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x1D  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpge_ospd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpge_osps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpge_ossd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpge_osss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpgepd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpgeps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpgesd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpgess" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0D  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpgt_oqpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x1E  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpgt_oqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x1E  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpgt_oqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x1E  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x1E  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpgt_oqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x1E  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x1E  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpgt_ospd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpgt_osps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpgt_ossd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpgt_osss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpgtpd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x0E  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpgtps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpgtsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpgtss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0E  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmple_oqpd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x12  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x12  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmple_oqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x12  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmple_oqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x12  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x12  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmple_oqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x12  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x12  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmple_ospd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmple_osps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmple_ossd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmple_osss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmplepd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x02  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpleps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmplesd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpless" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x02  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmplt_oqpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x11  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmplt_oqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x11  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmplt_oqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x11  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x11  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmplt_oqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x11  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x11  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmplt_ospd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x01  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmplt_osps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmplt_ossd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmplt_osss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpltpd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x01  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpltps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpltsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpltss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x01  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpneq_oqpd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x0C  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x0C  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpneq_oqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0C  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpneq_oqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0C  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0C  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpneq_oqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0C  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0C  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpneq_ospd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x1C  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x1C  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpneq_osps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x1C  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpneq_ossd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x1C  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x1C  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpneq_osss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x1C  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x1C  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpneq_uqpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpneq_uqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpneq_uqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpneq_uqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpneq_uspd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x14  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x14  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpneq_usps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x14  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpneq_ussd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x14  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x14  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpneq_usss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x14  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x14  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpneqpd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x04  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpneqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpneqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpneqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x04  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpnge_uqpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x19  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpnge_uqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x19  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnge_uqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x19  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x19  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpnge_uqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x19  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x19  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpnge_uspd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpnge_usps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnge_ussd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpnge_usss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpngepd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x09  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpngeps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpngesd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpngess" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x09  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpngt_uqpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x1A  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpngt_uqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x1A  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpngt_uqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x1A  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x1A  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpngt_uqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x1A  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x1A  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpngt_uspd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpngt_usps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpngt_ussd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpngt_usss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpngtpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpngtps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpngtsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpngtss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0A  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpnle_uqpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x16  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpnle_uqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x16  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnle_uqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x16  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x16  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpnle_uqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x16  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x16  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpnle_uspd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x06  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpnle_usps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnle_ussd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpnle_usss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpnlepd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpnleps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnlesd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpnless" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x06  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpnlt_uqpd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x15  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x15  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpnlt_uqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x15  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnlt_uqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x15  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x15  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpnlt_uqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x15  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x15  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpnlt_uspd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpnlt_usps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnlt_ussd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpnlt_usss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpnltpd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x05  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpnltps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpnltsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpnltss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x05  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpord_qpd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x07  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpord_qps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpord_qsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpord_qss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpord_spd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x17  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x17  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpord_sps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x17  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpord_ssd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x17  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x17  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpord_sss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x17  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x17  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpordpd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x07  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmpordps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpordsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpordss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x07  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmppd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                  ,  [0x01, 0xC2        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vcmpps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                  ,  [0x01, 0xC2        ], X, VEX_OP | AUTO_VEXL | ENC_MR, AVX;
]
"vcmpsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]      ,  [0x01, 0xC2        ], X, VEX_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]        ,  [0x01, 0xC2        ], X, VEX_OP | PREF_F2, AVX;
]
"vcmpss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]      ,  [0x01, 0xC2        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]        ,  [0x01, 0xC2        ], X, VEX_OP | PREF_F3, AVX;
]
"vcmptrue_uqpd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x0F  ], X, WITH_VEXL | VEX_OP | IMM_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vcmptrue_uqps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmptrue_uqsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmptrue_uqss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmptrue_uspd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x1F  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmptrue_usps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x1F  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmptrue_ussd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x1F  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x1F  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmptrue_usss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x1F  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x1F  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmptruepd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmptrueps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmptruesd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmptruess" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x0F  ], X, VEX_OP | PREF_F3 | IMM_OP, AVX;
]
"vcmpunord_qpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | AUTO_VEXL | PREF_66 | IMM_OP, AVX;
]
"vcmpunord_qps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpunord_qsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_F2, AVX;
]
"vcmpunord_qss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpunord_spd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x13  ], X, VEX_OP | AUTO_VEXL | IMM_OP | PREF_66, AVX;
]
"vcmpunord_sps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x13  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpunord_ssd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x13  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x13  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpunord_sss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x13  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x13  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcmpunordpd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x01, 0xC2, 0x03  ], X, WITH_VEXL | VEX_OP | PREF_66 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vcmpunordps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | AUTO_VEXL | IMM_OP, AVX;
]
"vcmpunordsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | PREF_F2 | IMM_OP, AVX;
]
"vcmpunordss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0xC2, 0x03  ], X, VEX_OP | IMM_OP | PREF_F3, AVX;
]
"vcomisd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x01, 0x2F        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x01, 0x2F        ], X, VEX_OP | PREF_66, AVX;
]
"vcomiss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x01, 0x2F        ], X, VEX_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x01, 0x2F        ], X, VEX_OP, AVX;
]
"vcvtdq2pd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x01, 0xE6        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                                                    ,  [0x01, 0xE6        ], X, VEX_OP | AUTO_VEXL | PREF_F3, AVX;
]
"vcvtdq2ps" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x5B        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vcvtpd2dq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::Auto]                                                                           ,  [0x01, 0xE6        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::Auto]                                                                             ,  [0x01, 0xE6        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
]
"vcvtpd2ps" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::Auto]                                                                           ,  [0x01, 0x5A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::Auto]                                                                             ,  [0x01, 0x5A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vcvtph2ps" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x02, 0x13        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                                                    ,  [0x02, 0x13        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vcvtps2dq" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x5B        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vcvtps2pd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x01, 0x5A        ], X, VEX_OP, AVX;
    [RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                                                    ,  [0x01, 0x5A        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vcvtps2ph" = [
    [OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                      ,  [0x03, 0x1D        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    [Size::Word, Size::OWord, RegisterType::AVX, Size::Auto, OperandType::Immediate, Size::Byte]                                                ,  [0x03, 0x1D        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vcvtsd2si" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::QWord]                                                                        ,  [0x01, 0x2D        ], X, VEX_OP | AUTO_REXW | PREF_F2, AVX;
    [RegisterType::Legacy, Size::Auto, RegisterType::AVX, Size::OWord]                                                                          ,  [0x01, 0x2D        ], X, VEX_OP | AUTO_REXW | PREF_F2, AVX;
]
"vcvtsd2ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0x5A        ], X, VEX_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x5A        ], X, VEX_OP | PREF_F2, AVX;
]
"vcvtsi2sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::Auto]                                     ,  [0x01, 0x2A        ], X, VEX_OP | AUTO_REXW | PREF_F2, AVX;
]
"vcvtsi2ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::Auto]                                     ,  [0x01, 0x2A        ], X, VEX_OP | AUTO_REXW | PREF_F3, AVX;
]
"vcvtss2sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x01, 0x5A        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x5A        ], X, VEX_OP | PREF_F3, AVX;
]
"vcvtss2si" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::DWord]                                                                        ,  [0x01, 0x2D        ], X, VEX_OP | AUTO_REXW | PREF_F3, AVX;
    [RegisterType::Legacy, Size::Auto, RegisterType::AVX, Size::OWord]                                                                          ,  [0x01, 0x2D        ], X, VEX_OP | AUTO_REXW | PREF_F3, AVX;
]
"vcvttpd2dq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::Auto]                                                                           ,  [0x01, 0xE6        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::Auto]                                                                             ,  [0x01, 0xE6        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vcvttps2dq" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x5B        ], X, VEX_OP | AUTO_VEXL | PREF_F3, AVX;
]
"vcvttsd2si" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::QWord]                                                                        ,  [0x01, 0x2C        ], X, VEX_OP | AUTO_REXW | PREF_F2, AVX;
    [RegisterType::Legacy, Size::Auto, RegisterType::AVX, Size::OWord]                                                                          ,  [0x01, 0x2C        ], X, VEX_OP | AUTO_REXW | PREF_F2, AVX;
]
"vcvttss2si" = [
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::DWord]                                                                        ,  [0x01, 0x2C        ], X, VEX_OP | AUTO_REXW | PREF_F3, AVX;
    [RegisterType::Legacy, Size::Auto, RegisterType::AVX, Size::OWord]                                                                          ,  [0x01, 0x2C        ], X, VEX_OP | AUTO_REXW | PREF_F3, AVX;
]
"vdivpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x5E        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vdivps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x5E        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vdivsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0x5E        ], X, VEX_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x5E        ], X, VEX_OP | PREF_F2, AVX;
]
"vdivss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x01, 0x5E        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x5E        ], X, VEX_OP | PREF_F3, AVX;
]
"vdppd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]               ,  [0x03, 0x41        ], X, VEX_OP | PREF_66, AVX;
]
"vdpps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                  ,  [0x03, 0x40        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"verr" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0x00        ], 4;
    [RegisterType::Legacy, Size::Word]                                                                                                          ,  [0x0F, 0x00        ], 4;
]
"verw" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0x00        ], 5;
    [RegisterType::Legacy, Size::Word]                                                                                                          ,  [0x0F, 0x00        ], 5;
]
"vextractf128" = [
    [Size::Word, Size::OWord, RegisterType::AVX, Size::HWord, OperandType::Immediate, Size::Byte]                                               ,  [0x03, 0x19        ], X, WITH_VEXL | VEX_OP | ENC_MR | PREF_66, AVX;
]
"vextracti128" = [
    [Size::Word, Size::OWord, RegisterType::AVX, Size::HWord, OperandType::Immediate, Size::Byte]                                               ,  [0x03, 0x39        ], X, WITH_VEXL | VEX_OP | ENC_MR | PREF_66, AVX2;
]
"vextractps" = [
    [OperandType::LegacyMemory, Size::DWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                ,  [0x03, 0x17        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vfmadd123pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xA8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd123ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xA8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd123sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xA9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xA9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmadd123ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xA9        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xA9        ], X, VEX_OP | PREF_66, FMA;
]
"vfmadd132pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x98        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd132ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x98        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd132sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0x99        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x99        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmadd132ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0x99        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x99        ], X, VEX_OP | PREF_66, FMA;
]
"vfmadd213pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xA8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd213ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xA8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd213sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xA9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xA9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmadd213ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xA9        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xA9        ], X, VEX_OP | PREF_66, FMA;
]
"vfmadd231pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xB8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd231ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xB8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd231sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xB9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xB9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmadd231ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xB9        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xB9        ], X, VEX_OP | PREF_66, FMA;
]
"vfmadd312pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x98        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd312ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x98        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd312sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0x99        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x99        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmadd312ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0x99        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x99        ], X, VEX_OP | PREF_66, FMA;
]
"vfmadd321pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xB8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd321ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xB8        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmadd321sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xB9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xB9        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmadd321ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xB9        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xB9        ], X, VEX_OP | PREF_66, FMA;
]
"vfmaddpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                       ,  [0x03, 0x69        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x69        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
]
"vfmaddps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                       ,  [0x03, 0x68        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x68        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
]
"vfmaddsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]          ,  [0x03, 0x6B        ], X, VEX_OP | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]          ,  [0x03, 0x6B        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]            ,  [0x03, 0x6B        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
]
"vfmaddss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord, RegisterType::AVX, Size::OWord]          ,  [0x03, 0x6A        ], X, VEX_OP | PREF_66, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]          ,  [0x03, 0x6A        ], X, VEX_OP | WITH_REXW | PREF_66, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]            ,  [0x03, 0x6A        ], X, VEX_OP | WITH_REXW | PREF_66, SSE5 | AMD;
]
"vfmaddsub123pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xA6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub123ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xA6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub132pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x96        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub132ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x96        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub213pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xA6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub213ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xA6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub231pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xB6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub231ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xB6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub312pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x96        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub312ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x96        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub321pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xB6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsub321ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xB6        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmaddsubpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                       ,  [0x03, 0x5D        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x5D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfmaddsubps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                       ,  [0x03, 0x5C        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x5C        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
]
"vfmsub123pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xAA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub123ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xAA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub123sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xAB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xAB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmsub123ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xAB        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xAB        ], X, VEX_OP | PREF_66, FMA;
]
"vfmsub132pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x9A        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub132ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x9A        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub132sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0x9B        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x9B        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmsub132ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0x9B        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x9B        ], X, VEX_OP | PREF_66, FMA;
]
"vfmsub213pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xAA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub213ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xAA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub213sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xAB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xAB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmsub213ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xAB        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xAB        ], X, VEX_OP | PREF_66, FMA;
]
"vfmsub231pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xBA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub231ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xBA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub231sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xBB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xBB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmsub231ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xBB        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xBB        ], X, VEX_OP | PREF_66, FMA;
]
"vfmsub312pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x9A        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub312ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x9A        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub312sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0x9B        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x9B        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmsub312ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0x9B        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x9B        ], X, VEX_OP | PREF_66, FMA;
]
"vfmsub321pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xBA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub321ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xBA        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsub321sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xBB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xBB        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfmsub321ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xBB        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xBB        ], X, VEX_OP | PREF_66, FMA;
]
"vfmsubadd123pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xA7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd123ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xA7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd132pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x97        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd132ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x97        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd213pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xA7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd213ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xA7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd231pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xB7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd231ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xB7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd312pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x97        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd312ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x97        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd321pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xB7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubadd321ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xB7        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfmsubaddpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                       ,  [0x03, 0x5F        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x5F        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfmsubaddps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                       ,  [0x03, 0x5E        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x5E        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfmsubpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                       ,  [0x03, 0x6D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x6D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfmsubps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                       ,  [0x03, 0x6C        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x6C        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
]
"vfmsubsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]          ,  [0x03, 0x6F        ], X, VEX_OP | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]          ,  [0x03, 0x6F        ], X, VEX_OP | WITH_REXW | PREF_66, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]            ,  [0x03, 0x6F        ], X, VEX_OP | WITH_REXW | PREF_66, SSE5 | AMD;
]
"vfmsubss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord, RegisterType::AVX, Size::OWord]          ,  [0x03, 0x6E        ], X, VEX_OP | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]          ,  [0x03, 0x6E        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]            ,  [0x03, 0x6E        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
]
"vfnmadd123pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xAC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd123ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xAC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd123sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xAD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xAD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmadd123ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xAD        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xAD        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmadd132pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x9C        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd132ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x9C        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd132sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0x9D        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x9D        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmadd132ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0x9D        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x9D        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmadd213pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xAC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd213ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xAC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd213sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xAD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xAD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmadd213ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xAD        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xAD        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmadd231pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xBC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd231ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xBC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd231sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xBD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xBD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmadd231ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xBD        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xBD        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmadd312pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x9C        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd312ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x9C        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd312sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0x9D        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x9D        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmadd312ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0x9D        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x9D        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmadd321pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xBC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd321ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xBC        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmadd321sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xBD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xBD        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmadd321ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xBD        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xBD        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmaddpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                       ,  [0x03, 0x79        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x79        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfnmaddps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                       ,  [0x03, 0x78        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x78        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
]
"vfnmaddsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]          ,  [0x03, 0x7B        ], X, VEX_OP | PREF_66, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]          ,  [0x03, 0x7B        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]            ,  [0x03, 0x7B        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
]
"vfnmaddss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord, RegisterType::AVX, Size::OWord]          ,  [0x03, 0x7A        ], X, VEX_OP | PREF_66, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]          ,  [0x03, 0x7A        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]            ,  [0x03, 0x7A        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
]
"vfnmsub123pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xAE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub123ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xAE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub123sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xAF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xAF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmsub123ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xAF        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xAF        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmsub132pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x9E        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub132ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x9E        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub132sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0x9F        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x9F        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmsub132ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0x9F        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x9F        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmsub213pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xAE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub213ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xAE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub213sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xAF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xAF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmsub213ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xAF        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xAF        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmsub231pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xBE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub231ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xBE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub231sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xBF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xBF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmsub231ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xBF        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xBF        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmsub312pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x9E        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub312ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x9E        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub312sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0x9F        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x9F        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmsub312ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0x9F        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0x9F        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmsub321pd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xBE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub321ps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0xBE        ], X, VEX_OP | AUTO_VEXL | PREF_66, FMA;
]
"vfnmsub321sd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x02, 0xBF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xBF        ], X, VEX_OP | WITH_REXW | PREF_66, FMA;
]
"vfnmsub321ss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x02, 0xBF        ], X, VEX_OP | PREF_66, FMA;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x02, 0xBF        ], X, VEX_OP | PREF_66, FMA;
]
"vfnmsubpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                       ,  [0x03, 0x7D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x7D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfnmsubps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                       ,  [0x03, 0x7C        ], X, VEX_OP | AUTO_VEXL | PREF_66, SSE5 | AMD;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x7C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AMD | SSE5;
]
"vfnmsubsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]          ,  [0x03, 0x7F        ], X, VEX_OP | PREF_66, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]          ,  [0x03, 0x7F        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]            ,  [0x03, 0x7F        ], X, VEX_OP | WITH_REXW | PREF_66, AMD | SSE5;
]
"vfnmsubss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord, RegisterType::AVX, Size::OWord]          ,  [0x03, 0x7E        ], X, VEX_OP | PREF_66, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]          ,  [0x03, 0x7E        ], X, VEX_OP | WITH_REXW | PREF_66, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]            ,  [0x03, 0x7E        ], X, VEX_OP | WITH_REXW | PREF_66, SSE5 | AMD;
]
"vfrczpd" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x09, 0x81        ], X, XOP_OP | AUTO_VEXL, SSE5 | AMD;
]
"vfrczps" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x09, 0x80        ], X, XOP_OP | AUTO_VEXL, AMD | SSE5;
]
"vfrczsd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x09, 0x83        ], X, XOP_OP, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x09, 0x83        ], X, XOP_OP, SSE5 | AMD;
]
"vfrczss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x09, 0x82        ], X, XOP_OP, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x09, 0x82        ], X, XOP_OP, AMD | SSE5;
]
"vgatherdpd" = [
    [RegisterType::AVX, Size::Auto, OperandType::VSIB64, Size::OWord, RegisterType::AVX, Size::Auto]                                            ,  [0x02, 0x92        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vgatherdps" = [
    [RegisterType::AVX, Size::Auto, OperandType::VSIB32, Size::Auto, RegisterType::AVX, Size::Auto]                                             ,  [0x02, 0x92        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vgatherqpd" = [
    [RegisterType::AVX, Size::Auto, OperandType::VSIB64, Size::Auto, RegisterType::AVX, Size::Auto]                                             ,  [0x02, 0x93        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vgatherqps" = [
    [RegisterType::AVX, Size::OWord, OperandType::VSIB32, Size::Auto, RegisterType::AVX, Size::OWord]                                           ,  [0x02, 0x93        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vhaddpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x7C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vhaddps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x7C        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
]
"vhsubpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x7D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vhsubps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x7D        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
]
"vinsertf128" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]               ,  [0x03, 0x18        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX;
]
"vinserti128" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]               ,  [0x03, 0x38        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX2;
]
"vinsertps" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord, OperandType::Immediate, Size::Byte]      ,  [0x03, 0x21        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]        ,  [0x03, 0x21        ], X, VEX_OP | PREF_66, AVX;
]
"vlddqu" = [
    [RegisterType::AVX, Size::Auto, OperandType::Memory, Size::Auto]                                                                            ,  [0x01, 0xF0        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
]
"vldmxcsr" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0x01, 0xAE        ], 2, VEX_OP, AVX;
]
"vldqqu" = [
    [RegisterType::AVX, Size::HWord, OperandType::Memory, Size::HWord]                                                                          ,  [0x01, 0xF0        ], X, WITH_VEXL | VEX_OP | PREF_F2, AVX;
]
"vmaskmovdqu" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x01, 0xF7        ], X, VEX_OP | PREF_66, AVX;
]
"vmaskmovpd" = [
    [OperandType::Memory, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto]                                             ,  [0x02, 0x2F        ], X, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Memory, Size::Auto]                                             ,  [0x02, 0x2D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vmaskmovps" = [
    [OperandType::Memory, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto]                                             ,  [0x02, 0x2E        ], X, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Memory, Size::Auto]                                             ,  [0x02, 0x2C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vmaxpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x5F        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vmaxps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x5F        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vmaxsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0x5F        ], X, VEX_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x5F        ], X, VEX_OP | PREF_F2, AVX;
]
"vmaxss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x01, 0x5F        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x5F        ], X, VEX_OP | PREF_F3, AVX;
]
"vmcall" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xC1  ], X, DEFAULT, VMX;
]
"vmclear" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xC7        ], 6, PREF_66, VMX;
]
"vmfunc" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xD4  ], X, DEFAULT, VMX;
]
"vminpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x5D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vminps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x5D        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vminsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0x5D        ], X, VEX_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x5D        ], X, VEX_OP | PREF_F2, AVX;
]
"vminss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x01, 0x5D        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x5D        ], X, VEX_OP | PREF_F3, AVX;
]
"vmlaunch" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xC2  ], X, DEFAULT, VMX;
]
"vmload" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xDA  ], X, DEFAULT, AMD | VMX;
]
"vmmcall" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xD9  ], X, DEFAULT, AMD | VMX;
]
"vmovapd" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x28        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    [Size::Word, Size::HWord, RegisterType::AVX, Size::HWord]                                                                                   ,  [0x01, 0x29        ], X, VEX_OP | WITH_VEXL | ENC_MR | PREF_66, AVX;
    [Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                                                   ,  [0x01, 0x29        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovaps" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x28        ], X, VEX_OP | AUTO_VEXL, AVX;
    [Size::Word, Size::HWord, RegisterType::AVX, Size::HWord]                                                                                   ,  [0x01, 0x29        ], X, VEX_OP | WITH_VEXL | ENC_MR, AVX;
    [Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                                                   ,  [0x01, 0x29        ], X, VEX_OP | ENC_MR, AVX;
]
"vmovd" = [
    [RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::DWord]                                                                    ,  [0x01, 0x6E        ], X, VEX_OP | PREF_66, AVX;
    [OperandType::LegacyMemory, Size::DWord, RegisterType::AVX, Size::OWord]                                                                    ,  [0x01, 0x7E        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovddup" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x12        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x01, 0x12        ], X, VEX_OP | PREF_F2, AVX;
]
"vmovdqa" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x6F        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    [Size::Word, Size::HWord, RegisterType::AVX, Size::HWord]                                                                                   ,  [0x01, 0x7F        ], X, VEX_OP | WITH_VEXL | ENC_MR | PREF_66, AVX;
    [Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                                                   ,  [0x01, 0x7F        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovdqu" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x6F        ], X, VEX_OP | AUTO_VEXL | PREF_F3, AVX;
    [Size::Word, Size::HWord, RegisterType::AVX, Size::HWord]                                                                                   ,  [0x01, 0x7F        ], X, VEX_OP | WITH_VEXL | ENC_MR | PREF_F3, AVX;
    [Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                                                   ,  [0x01, 0x7F        ], X, VEX_OP | ENC_MR | PREF_F3, AVX;
]
"vmovhlps" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x12        ], X, VEX_OP, AVX;
]
"vmovhpd" = [
    [OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x01, 0x17        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0x16        ], X, VEX_OP | PREF_66, AVX;
]
"vmovhps" = [
    [OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x01, 0x17        ], X, VEX_OP | ENC_MR, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0x16        ], X, VEX_OP, AVX;
]
"vmovlhps" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x16        ], X, VEX_OP, AVX;
]
"vmovlpd" = [
    [OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x01, 0x13        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0x12        ], X, VEX_OP | PREF_66, AVX;
]
"vmovlps" = [
    [OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x01, 0x13        ], X, VEX_OP | ENC_MR, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0x12        ], X, VEX_OP, AVX;
]
"vmovmskpd" = [
    [RegisterType::Legacy, Size::Auto, RegisterType::AVX, Size::Auto]                                                                           ,  [0x01, 0x50        ], X, VEX_OP | PREF_66, AVX;
]
"vmovmskps" = [
    [RegisterType::Legacy, Size::Auto, RegisterType::AVX, Size::Auto]                                                                           ,  [0x01, 0x50        ], X, VEX_OP, AVX;
]
"vmovntdq" = [
    [OperandType::Memory, Size::Auto, RegisterType::AVX, Size::Auto]                                                                            ,  [0x01, 0xE7        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vmovntdqa" = [
    [RegisterType::AVX, Size::Auto, OperandType::Memory, Size::Auto]                                                                            ,  [0x02, 0x2A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vmovntpd" = [
    [OperandType::Memory, Size::Auto, RegisterType::AVX, Size::Auto]                                                                            ,  [0x01, 0x2B        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vmovntps" = [
    [OperandType::Memory, Size::Auto, RegisterType::AVX, Size::Auto]                                                                            ,  [0x01, 0x2B        ], X, VEX_OP | AUTO_VEXL | ENC_MR, AVX;
]
"vmovntqq" = [
    [OperandType::Memory, Size::HWord, RegisterType::AVX, Size::HWord]                                                                          ,  [0x01, 0xE7        ], X, WITH_VEXL | VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovq" = [
    [OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x01, 0xD6        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x01, 0x7E        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::QWord]                                                                    ,  [0x01, 0x6E        ], X, WITH_REXW | VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x01, 0x7E        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x01, 0xD6        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    [OperandType::LegacyMemory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                    ,  [0x01, 0x7E        ], X, WITH_REXW | VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovqqa" = [
    [RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                                                   ,  [0x01, 0x6F        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX;
    [Size::Word, Size::HWord, RegisterType::AVX, Size::HWord]                                                                                   ,  [0x01, 0x7F        ], X, WITH_VEXL | VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovqqu" = [
    [RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                                                   ,  [0x01, 0x6F        ], X, WITH_VEXL | VEX_OP | PREF_F3, AVX;
    [Size::Word, Size::HWord, RegisterType::AVX, Size::HWord]                                                                                   ,  [0x01, 0x7F        ], X, WITH_VEXL | VEX_OP | ENC_MR | PREF_F3, AVX;
]
"vmovsd" = [
    [OperandType::Memory, Size::QWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x01, 0x11        ], X, VEX_OP | ENC_MR | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x01, 0x10        ], X, VEX_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x10        ], X, VEX_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x11        ], X, VEX_OP | ENC_VM | PREF_F2, AVX;
]
"vmovshdup" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x16        ], X, VEX_OP | AUTO_VEXL | PREF_F3, AVX;
]
"vmovsldup" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x12        ], X, VEX_OP | AUTO_VEXL | PREF_F3, AVX;
]
"vmovss" = [
    [OperandType::Memory, Size::DWord, RegisterType::AVX, Size::OWord]                                                                          ,  [0x01, 0x11        ], X, VEX_OP | ENC_MR | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x01, 0x10        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x10        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x11        ], X, VEX_OP | ENC_VM | PREF_F3, AVX;
]
"vmovupd" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x10        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    [Size::Word, Size::HWord, RegisterType::AVX, Size::HWord]                                                                                   ,  [0x01, 0x11        ], X, VEX_OP | WITH_VEXL | ENC_MR | PREF_66, AVX;
    [Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                                                   ,  [0x01, 0x11        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vmovups" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x10        ], X, VEX_OP | AUTO_VEXL, AVX;
    [Size::Word, Size::HWord, RegisterType::AVX, Size::HWord]                                                                                   ,  [0x01, 0x11        ], X, VEX_OP | WITH_VEXL | ENC_MR, AVX;
    [Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                                                   ,  [0x01, 0x11        ], X, VEX_OP | ENC_MR, AVX;
]
"vmpsadbw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                  ,  [0x03, 0x42        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vmptrld" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xC7        ], 6, DEFAULT, VMX;
]
"vmptrst" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xC7        ], 7, DEFAULT, VMX;
]
"vmread" = [
    [OperandType::LegacyMemory, Size::QWord, RegisterType::Legacy, Size::QWord]                                                                 ,  [0x0F, 0x78        ], X, ENC_MR, VMX;
]
"vmresume" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xC3  ], X, DEFAULT, VMX;
]
"vmrun" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xD8  ], X, DEFAULT, AMD | VMX;
]
"vmsave" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xDB  ], X, DEFAULT, VMX | AMD;
]
"vmulpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x59        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vmulps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x59        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vmulsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0x59        ], X, VEX_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x59        ], X, VEX_OP | PREF_F2, AVX;
]
"vmulss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x01, 0x59        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x59        ], X, VEX_OP | PREF_F3, AVX;
]
"vmwrite" = [
    [RegisterType::Legacy, Size::QWord, OperandType::LegacyMemory, Size::QWord]                                                                 ,  [0x0F, 0x79        ], X, DEFAULT, VMX;
]
"vmxoff" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xC4  ], X, DEFAULT, VMX;
]
"vmxon" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xC7        ], 6, PREF_F3, VMX;
]
"vorpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x56        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vorps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x56        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vpabsb" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x02, 0x1C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpabsd" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x02, 0x1E        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpabsw" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x02, 0x1D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpackssdw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x6B        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpacksswb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x63        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpackusdw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x2B        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpackuswb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x67        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xFC        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xFE        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xD4        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddsb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xEC        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddsw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xED        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddusb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xDC        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddusw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xDD        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpaddw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xFD        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpalignr" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                  ,  [0x03, 0x0F        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vpand" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xDB        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpandn" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xDF        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpavgb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xE0        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpavgw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xE3        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpblendd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                  ,  [0x03, 0x02        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vpblendvb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x03, 0x4C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpblendw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                  ,  [0x03, 0x0E        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vpbroadcastb" = [
    [RegisterType::AVX, Size::Auto, OperandType::Memory, Size::Byte]                                                                            ,  [0x02, 0x78        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::OWord]                                                                             ,  [0x02, 0x78        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpbroadcastd" = [
    [RegisterType::AVX, Size::Auto, OperandType::Memory, Size::DWord]                                                                           ,  [0x02, 0x58        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::OWord]                                                                             ,  [0x02, 0x58        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpbroadcastq" = [
    [RegisterType::AVX, Size::HWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x02, 0x59        ], X, VEX_OP | WITH_VEXL | PREF_66, AVX2;
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x02, 0x59        ], X, VEX_OP | PREF_66, AVX2;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::OWord]                                                                             ,  [0x02, 0x59        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpbroadcastw" = [
    [RegisterType::AVX, Size::Auto, OperandType::Memory, Size::Word]                                                                            ,  [0x02, 0x79        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::OWord]                                                                             ,  [0x02, 0x79        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpclmulhqhqdq" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x03, 0x44, 0x11  ], X, VEX_OP | PREF_66 | IMM_OP, AVX;
]
"vpclmulhqlqdq" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x03, 0x44, 0x01  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vpclmullqhqdq" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x03, 0x44, 0x10  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vpclmullqlqdq" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x03, 0x44, 0x00  ], X, VEX_OP | IMM_OP | PREF_66, AVX;
]
"vpclmulqdq" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]               ,  [0x03, 0x44        ], X, VEX_OP | PREF_66, AVX;
]
"vpcmov" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, RegisterType::AVX, Size::Auto]                       ,  [0x08, 0xA2        ], X, XOP_OP | AUTO_VEXL, SSE5 | AMD;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                       ,  [0x08, 0xA2        ], X, XOP_OP | AUTO_VEXL, AMD | SSE5;
]
"vpcmpeqb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x74        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpeqd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x76        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpeqq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x29        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpeqw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x75        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpestri" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x03, 0x61        ], X, VEX_OP | PREF_66, AVX;
]
"vpcmpestrm" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x03, 0x60        ], X, VEX_OP | PREF_66, AVX;
]
"vpcmpgtb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x64        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpgtd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x66        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpgtq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x37        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpgtw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x65        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpcmpistri" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x03, 0x63        ], X, VEX_OP | PREF_66, AVX;
]
"vpcmpistrm" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x03, 0x62        ], X, VEX_OP | PREF_66, AVX;
]
"vpcomb" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]               ,  [0x08, 0xCC        ], X, XOP_OP, AMD | SSE5;
]
"vpcomd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]               ,  [0x08, 0xCE        ], X, XOP_OP, AMD | SSE5;
]
"vpcomq" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]               ,  [0x08, 0xCF        ], X, XOP_OP, SSE5 | AMD;
]
"vpcomub" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]               ,  [0x08, 0xEC        ], X, XOP_OP, AMD | SSE5;
]
"vpcomud" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]               ,  [0x08, 0xEE        ], X, XOP_OP, SSE5 | AMD;
]
"vpcomuq" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]               ,  [0x08, 0xEF        ], X, XOP_OP, AMD | SSE5;
]
"vpcomuw" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]               ,  [0x08, 0xED        ], X, XOP_OP, SSE5 | AMD;
]
"vpcomw" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]               ,  [0x08, 0xCD        ], X, XOP_OP, AMD | SSE5;
]
"vperm2f128" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord, OperandType::Immediate, Size::Byte]               ,  [0x03, 0x06        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX;
]
"vperm2i128" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord, OperandType::Immediate, Size::Byte]               ,  [0x03, 0x46        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX2;
]
"vpermd" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x02, 0x36        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX2;
]
"vpermilpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x0D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                                                 ,  [0x03, 0x05        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpermilps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x0C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                                                 ,  [0x03, 0x04        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpermpd" = [
    [RegisterType::AVX, Size::HWord, Size::Word, Size::HWord, OperandType::Immediate, Size::Byte]                                               ,  [0x03, 0x01        ], X, WITH_VEXL | WITH_REXW | VEX_OP | PREF_66, AVX2;
]
"vpermps" = [
    [RegisterType::AVX, Size::HWord, RegisterType::AVX, Size::HWord, Size::Word, Size::HWord]                                                   ,  [0x02, 0x16        ], X, WITH_VEXL | VEX_OP | PREF_66, AVX2;
]
"vpermq" = [
    [RegisterType::AVX, Size::HWord, Size::Word, Size::HWord, OperandType::Immediate, Size::Byte]                                               ,  [0x03, 0x00        ], X, WITH_VEXL | WITH_REXW | VEX_OP | PREF_66, AVX2;
]
"vpextrb" = [
    [OperandType::Memory, Size::Byte, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                       ,  [0x03, 0x14        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    [RegisterType::Legacy, Size::DWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                     ,  [0x03, 0x14        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    [RegisterType::Legacy, Size::QWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                     ,  [0x03, 0x14        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vpextrd" = [
    [RegisterType::Legacy, Size::QWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                     ,  [0x03, 0x16        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    [OperandType::LegacyMemory, Size::DWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                ,  [0x03, 0x16        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vpextrq" = [
    [OperandType::LegacyMemory, Size::QWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                ,  [0x03, 0x16        ], X, WITH_REXW | VEX_OP | ENC_MR | PREF_66, AVX;
]
"vpextrw" = [
    [OperandType::Memory, Size::Word, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                       ,  [0x03, 0x15        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    [RegisterType::Legacy, Size::DWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                     ,  [0x01, 0xC5        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::Legacy, Size::DWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                     ,  [0x03, 0x15        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
    [RegisterType::Legacy, Size::QWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                     ,  [0x01, 0xC5        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::Legacy, Size::QWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]                                     ,  [0x03, 0x15        ], X, VEX_OP | ENC_MR | PREF_66, AVX;
]
"vpgatherdd" = [
    [RegisterType::AVX, Size::Auto, OperandType::VSIB32, Size::Auto, RegisterType::AVX, Size::Auto]                                             ,  [0x02, 0x90        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vpgatherdq" = [
    [RegisterType::AVX, Size::Auto, OperandType::VSIB64, Size::OWord, RegisterType::AVX, Size::Auto]                                            ,  [0x02, 0x90        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vpgatherqd" = [
    [RegisterType::AVX, Size::OWord, OperandType::VSIB32, Size::Auto, RegisterType::AVX, Size::OWord]                                           ,  [0x02, 0x91        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vpgatherqq" = [
    [RegisterType::AVX, Size::Auto, OperandType::VSIB64, Size::Auto, RegisterType::AVX, Size::Auto]                                             ,  [0x02, 0x91        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX2;
]
"vphaddbd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xC2        ], X, XOP_OP, SSE5 | AMD;
]
"vphaddbq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xC3        ], X, XOP_OP, SSE5 | AMD;
]
"vphaddbw" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xC1        ], X, XOP_OP, SSE5 | AMD;
]
"vphaddd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x02        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vphadddq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xCB        ], X, XOP_OP, SSE5 | AMD;
]
"vphaddsw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x03        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vphaddubd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xD2        ], X, XOP_OP, SSE5 | AMD;
]
"vphaddubq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xD3        ], X, XOP_OP, AMD | SSE5;
]
"vphaddubw" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xD1        ], X, XOP_OP, SSE5 | AMD;
]
"vphaddudq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xDB        ], X, XOP_OP, AMD | SSE5;
]
"vphadduwd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xD6        ], X, XOP_OP, AMD | SSE5;
]
"vphadduwq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xD7        ], X, XOP_OP, AMD | SSE5;
]
"vphaddw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x01        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vphaddwd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xC6        ], X, XOP_OP, AMD | SSE5;
]
"vphaddwq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xC7        ], X, XOP_OP, AMD | SSE5;
]
"vphminposuw" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x02, 0x41        ], X, VEX_OP | PREF_66, AVX;
]
"vphsubbw" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xE1        ], X, XOP_OP, SSE5 | AMD;
]
"vphsubd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x06        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vphsubdq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xE3        ], X, XOP_OP, SSE5 | AMD;
]
"vphsubsw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x07        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vphsubw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x05        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vphsubwd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x09, 0xE2        ], X, XOP_OP, SSE5 | AMD;
]
"vpinsrb" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::Legacy, Size::DWord, OperandType::Immediate, Size::Byte]     ,  [0x03, 0x20        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::Byte, OperandType::Immediate, Size::Byte] ,  [0x03, 0x20        ], X, VEX_OP | PREF_66, AVX;
]
"vpinsrd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::DWord, OperandType::Immediate, Size::Byte],  [0x03, 0x22        ], X, VEX_OP | PREF_66, AVX;
]
"vpinsrq" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::QWord, OperandType::Immediate, Size::Byte],  [0x03, 0x22        ], X, VEX_OP | WITH_REXW | PREF_66, AVX;
]
"vpinsrw" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::Legacy, Size::DWord, OperandType::Immediate, Size::Byte]     ,  [0x01, 0xC4        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::LegacyMemory, Size::Word, OperandType::Immediate, Size::Byte] ,  [0x01, 0xC4        ], X, VEX_OP | PREF_66, AVX;
]
"vpmacsdd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                   ,  [0x08, 0x9E        ], X, XOP_OP, AMD | SSE5;
]
"vpmacsdqh" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                   ,  [0x08, 0x9F        ], X, XOP_OP, SSE5 | AMD;
]
"vpmacsdql" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                   ,  [0x08, 0x97        ], X, XOP_OP, AMD | SSE5;
]
"vpmacssdd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                   ,  [0x08, 0x8E        ], X, XOP_OP, SSE5 | AMD;
]
"vpmacssdqh" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                   ,  [0x08, 0x8F        ], X, XOP_OP, AMD | SSE5;
]
"vpmacssdql" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                   ,  [0x08, 0x87        ], X, XOP_OP, SSE5 | AMD;
]
"vpmacsswd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                   ,  [0x08, 0x86        ], X, XOP_OP, AMD | SSE5;
]
"vpmacssww" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                   ,  [0x08, 0x85        ], X, XOP_OP, AMD | SSE5;
]
"vpmacswd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                   ,  [0x08, 0x96        ], X, XOP_OP, SSE5 | AMD;
]
"vpmacsww" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                   ,  [0x08, 0x95        ], X, XOP_OP, AMD | SSE5;
]
"vpmadcsswd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                   ,  [0x08, 0xA6        ], X, XOP_OP, SSE5 | AMD;
]
"vpmadcswd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                   ,  [0x08, 0xB6        ], X, XOP_OP, AMD | SSE5;
]
"vpmaddubsw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x04        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaddwd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xF5        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaskmovd" = [
    [OperandType::Memory, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto]                                             ,  [0x02, 0x8E        ], X, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX2;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Memory, Size::Auto]                                             ,  [0x02, 0x8C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpmaskmovq" = [
    [OperandType::Memory, Size::Auto, RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto]                                             ,  [0x02, 0x8E        ], X, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX2;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Memory, Size::Auto]                                             ,  [0x02, 0x8C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpmaxsb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x3C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaxsd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x3D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaxsw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xEE        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaxub" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xDE        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaxud" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x3F        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmaxuw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x3E        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpminsb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x38        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpminsd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x39        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpminsw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xEA        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpminub" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xDA        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpminud" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x3B        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpminuw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x3A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovmskb" = [
    [RegisterType::Legacy, Size::Auto, RegisterType::AVX, Size::Auto]                                                                           ,  [0x01, 0xD7        ], X, VEX_OP | PREF_66, AVX;
]
"vpmovsxbd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x02, 0x21        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::OWord]                                                                             ,  [0x02, 0x21        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovsxbq" = [
    [RegisterType::AVX, Size::Auto, OperandType::Memory, Size::Word]                                                                            ,  [0x02, 0x22        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::OWord]                                                                             ,  [0x02, 0x22        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovsxbw" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x02, 0x20        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                                                    ,  [0x02, 0x20        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovsxdq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x02, 0x25        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                                                    ,  [0x02, 0x25        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovsxwd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x02, 0x23        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                                                    ,  [0x02, 0x23        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovsxwq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x02, 0x24        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::OWord]                                                                             ,  [0x02, 0x24        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovzxbd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x02, 0x31        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::OWord]                                                                             ,  [0x02, 0x31        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovzxbq" = [
    [RegisterType::AVX, Size::Auto, OperandType::Memory, Size::Word]                                                                            ,  [0x02, 0x32        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::OWord]                                                                             ,  [0x02, 0x32        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovzxbw" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x02, 0x30        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                                                    ,  [0x02, 0x30        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovzxdq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x02, 0x35        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                                                    ,  [0x02, 0x35        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovzxwd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x02, 0x33        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                                                    ,  [0x02, 0x33        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmovzxwq" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x02, 0x34        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::OWord]                                                                             ,  [0x02, 0x34        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmuldq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x28        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmulhrsw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x0B        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmulhuw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xE4        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmulhw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xE5        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmulld" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x40        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmullw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xD5        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpmuludq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xF4        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpor" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xEB        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpperm" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                   ,  [0x08, 0xA3        ], X, XOP_OP, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                   ,  [0x08, 0xA3        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vprotb" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x08, 0xC0        ], X, XOP_OP, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                   ,  [0x09, 0x90        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x09, 0x90        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vprotd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x08, 0xC2        ], X, XOP_OP, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                   ,  [0x09, 0x92        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x09, 0x92        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vprotq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x08, 0xC3        ], X, XOP_OP, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                   ,  [0x09, 0x93        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x09, 0x93        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vprotw" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, OperandType::Immediate, Size::Byte]                                               ,  [0x08, 0xC1        ], X, XOP_OP, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                   ,  [0x09, 0x91        ], X, XOP_OP | ENC_MR, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x09, 0x91        ], X, WITH_REXW | XOP_OP, AMD | SSE5;
]
"vpsadbw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xF6        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpshab" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                   ,  [0x09, 0x98        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x09, 0x98        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vpshad" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                   ,  [0x09, 0x9A        ], X, XOP_OP | ENC_MR, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x09, 0x9A        ], X, WITH_REXW | XOP_OP, AMD | SSE5;
]
"vpshaq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                   ,  [0x09, 0x9B        ], X, XOP_OP | ENC_MR, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x09, 0x9B        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vpshaw" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                   ,  [0x09, 0x99        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x09, 0x99        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vpshlb" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                   ,  [0x09, 0x94        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x09, 0x94        ], X, WITH_REXW | XOP_OP, AMD | SSE5;
]
"vpshld" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                   ,  [0x09, 0x96        ], X, XOP_OP | ENC_MR, SSE5 | AMD;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x09, 0x96        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vpshlq" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                   ,  [0x09, 0x97        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x09, 0x97        ], X, WITH_REXW | XOP_OP, AMD | SSE5;
]
"vpshlw" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord, RegisterType::AVX, Size::OWord]                                                   ,  [0x09, 0x95        ], X, XOP_OP | ENC_MR, AMD | SSE5;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                   ,  [0x09, 0x95        ], X, WITH_REXW | XOP_OP, SSE5 | AMD;
]
"vpshufb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x00        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpshufd" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                                                 ,  [0x01, 0x70        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpshufhw" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                                                 ,  [0x01, 0x70        ], X, VEX_OP | AUTO_VEXL | PREF_F3, AVX;
]
"vpshuflw" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                                                 ,  [0x01, 0x70        ], X, VEX_OP | AUTO_VEXL | PREF_F2, AVX;
]
"vpsignb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x08        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsignd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x0A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsignw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x09        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpslld" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Immediate, Size::Byte]                                          ,  [0x01, 0x72        ], 6, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                     ,  [0x01, 0xF2        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpslldq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Immediate, Size::Byte]                                          ,  [0x01, 0x73        ], 7, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
]
"vpsllq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Immediate, Size::Byte]                                          ,  [0x01, 0x73        ], 6, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                     ,  [0x01, 0xF3        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsllvd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x47        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpsllvq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x47        ], X, VEX_OP | AUTO_VEXL | WITH_REXW | PREF_66, AVX2;
]
"vpsllw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Immediate, Size::Byte]                                          ,  [0x01, 0x71        ], 6, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                     ,  [0x01, 0xF1        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsrad" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Immediate, Size::Byte]                                          ,  [0x01, 0x72        ], 4, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                     ,  [0x01, 0xE2        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsravd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x46        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpsraw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Immediate, Size::Byte]                                          ,  [0x01, 0x71        ], 4, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                     ,  [0x01, 0xE1        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsrld" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Immediate, Size::Byte]                                          ,  [0x01, 0x72        ], 2, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                     ,  [0x01, 0xD2        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsrldq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Immediate, Size::Byte]                                          ,  [0x01, 0x73        ], 3, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
]
"vpsrlq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Immediate, Size::Byte]                                          ,  [0x01, 0x73        ], 2, VEX_OP | ENC_VM | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                     ,  [0x01, 0xD3        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsrlvd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x45        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX2;
]
"vpsrlvq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x02, 0x45        ], X, VEX_OP | AUTO_VEXL | WITH_REXW | PREF_66, AVX2;
]
"vpsrlw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, OperandType::Immediate, Size::Byte]                                          ,  [0x01, 0x71        ], 2, VEX_OP | AUTO_VEXL | ENC_VM | PREF_66, AVX;
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::OWord]                                                     ,  [0x01, 0xD1        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xF8        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xFA        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xFB        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubsb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xE8        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubsw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xE9        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubusb" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xD8        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubusw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xD9        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpsubw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xF9        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vptest" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x02, 0x17        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpckhbw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x68        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpckhdq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x6A        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpckhqdq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x6D        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpckhwd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x69        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpcklbw" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x60        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpckldq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x62        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpcklqdq" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x6C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpunpcklwd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x61        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vpxor" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0xEF        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vrcpps" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x53        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vrcpss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x01, 0x53        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x53        ], X, VEX_OP | PREF_F3, AVX;
]
"vroundpd" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                                                 ,  [0x03, 0x09        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vroundps" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                                                 ,  [0x03, 0x08        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vroundsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord, OperandType::Immediate, Size::Byte]      ,  [0x03, 0x0B        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]        ,  [0x03, 0x0B        ], X, VEX_OP | PREF_66, AVX;
]
"vroundss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord, OperandType::Immediate, Size::Byte]      ,  [0x03, 0x0A        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Immediate, Size::Byte]        ,  [0x03, 0x0A        ], X, VEX_OP | PREF_66, AVX;
]
"vrsqrtps" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x52        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vrsqrtss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x01, 0x52        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x52        ], X, VEX_OP | PREF_F3, AVX;
]
"vshufpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                  ,  [0x01, 0xC6        ], X, VEX_OP | AUTO_VEXL | ENC_MR | PREF_66, AVX;
]
"vshufps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto, OperandType::Immediate, Size::Byte]                  ,  [0x01, 0xC6        ], X, VEX_OP | AUTO_VEXL | ENC_MR, AVX;
]
"vsqrtpd" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x51        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vsqrtps" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x01, 0x51        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vsqrtsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0x51        ], X, VEX_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x51        ], X, VEX_OP | PREF_F2, AVX;
]
"vsqrtss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x01, 0x51        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x51        ], X, VEX_OP | PREF_F3, AVX;
]
"vstmxcsr" = [
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0x01, 0xAE        ], 3, VEX_OP, AVX;
]
"vsubpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x5C        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vsubps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x5C        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vsubsd" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                          ,  [0x01, 0x5C        ], X, VEX_OP | PREF_F2, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x5C        ], X, VEX_OP | PREF_F2, AVX;
]
"vsubss" = [
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                          ,  [0x01, 0x5C        ], X, VEX_OP | PREF_F3, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                            ,  [0x01, 0x5C        ], X, VEX_OP | PREF_F3, AVX;
]
"vtestpd" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x02, 0x0F        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vtestps" = [
    [RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                                                     ,  [0x02, 0x0E        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vucomisd" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::QWord]                                                                          ,  [0x01, 0x2E        ], X, VEX_OP | PREF_66, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x01, 0x2E        ], X, VEX_OP | PREF_66, AVX;
]
"vucomiss" = [
    [RegisterType::AVX, Size::OWord, OperandType::Memory, Size::DWord]                                                                          ,  [0x01, 0x2E        ], X, VEX_OP, AVX;
    [RegisterType::AVX, Size::OWord, RegisterType::AVX, Size::OWord]                                                                            ,  [0x01, 0x2E        ], X, VEX_OP, AVX;
]
"vunpckhpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x15        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vunpckhps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x15        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vunpcklpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x14        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vunpcklps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x14        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vxorpd" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x57        ], X, VEX_OP | AUTO_VEXL | PREF_66, AVX;
]
"vxorps" = [
    [RegisterType::AVX, Size::Auto, RegisterType::AVX, Size::Auto, Size::Word, Size::Auto]                                                      ,  [0x01, 0x57        ], X, VEX_OP | AUTO_VEXL, AVX;
]
"vzeroall" = [
    []                                                                                                                                          ,  [0x01, 0x77        ], X, WITH_VEXL | VEX_OP, AVX;
]
"vzeroupper" = [
    []                                                                                                                                          ,  [0x01, 0x77        ], X, VEX_OP, AVX;
]
"wbinvd" = [
    []                                                                                                                                          ,  [0x0F, 0x09        ], X;
]
"wrfsbase" = [
    [RegisterType::Legacy, Size::DWord]                                                                                                         ,  [0x0F, 0xAE        ], 2, PREF_F3;
    [RegisterType::Legacy, Size::QWord]                                                                                                         ,  [0x0F, 0xAE        ], 2, WITH_REXW | PREF_F3;
]
"wrgsbase" = [
    [RegisterType::Legacy, Size::DWord]                                                                                                         ,  [0x0F, 0xAE        ], 3, PREF_F3;
    [RegisterType::Legacy, Size::QWord]                                                                                                         ,  [0x0F, 0xAE        ], 3, WITH_REXW | PREF_F3;
]
"wrmsr" = [
    []                                                                                                                                          ,  [0x0F, 0x30        ], X;
]
"wrpkru" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xEF  ], X;
]
"wrshr" = [
    [OperandType::LegacyMemory, Size::DWord]                                                                                                    ,  [0x0F, 0x37        ], 0, DEFAULT, CYRIX;
]
"xabort" = [
    [OperandType::Immediate, Size::Byte]                                                                                                        ,  [0xC6, 0xF8        ], X, DEFAULT, RTM;
]
"xadd" = [
    [OperandType::Memory, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                         ,  [0x0F, 0xC0        ], X, LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                        ,  [0x0F, 0xC0        ], X, ENC_MR;
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x0F, 0xC1        ], X, AUTO_SIZE | LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x0F, 0xC1        ], X, AUTO_SIZE | ENC_MR;
]
"xbegin" = [
    [Size::OWord, Size::DWord]                                                                                                                  ,  [0xC7, 0xF8        ], X, DEFAULT, RTM;
]
"xchg" = [
    [OperandType::Memory, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                         ,  [0x86              ], X, LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::Memory, Size::Byte]                                                                         ,  [0x86              ], X, LOCK;
    [RegisterType::Legacy, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                        ,  [0x86              ], X;
    [RegisterType::Legacy, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                        ,  [0x86              ], X, ENC_MR;
    [RegisterX64::rax, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                            ,  [0x90              ], X, AUTO_SIZE | SHORT_ARG;
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x87              ], X, AUTO_SIZE | ENC_MR;
    [RegisterType::Legacy, Size::Auto, RegisterX64::rax, Size::Auto]                                                                            ,  [0x90              ], X, AUTO_SIZE | SHORT_ARG;
    [RegisterType::Legacy, Size::Auto, OperandType::Memory, Size::Auto]                                                                         ,  [0x87              ], X, AUTO_SIZE;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x87              ], X, AUTO_SIZE;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x87              ], X, AUTO_SIZE | ENC_MR;
]
"xcryptcbc" = [
    []                                                                                                                                          ,  [0x0F, 0xA7, 0xD0  ], X, PREF_F3, CYRIX;
]
"xcryptcfb" = [
    []                                                                                                                                          ,  [0x0F, 0xA7, 0xE0  ], X, PREF_F3, CYRIX;
]
"xcryptctr" = [
    []                                                                                                                                          ,  [0x0F, 0xA7, 0xD8  ], X, PREF_F3, CYRIX;
]
"xcryptecb" = [
    []                                                                                                                                          ,  [0x0F, 0xA7, 0xC8  ], X, PREF_F3, CYRIX;
]
"xcryptofb" = [
    []                                                                                                                                          ,  [0x0F, 0xA7, 0xE8  ], X, PREF_F3, CYRIX;
]
"xend" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xD5  ], X, DEFAULT, RTM;
]
"xgetbv" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xD0  ], X;
]
"xlat" = [
    []                                                                                                                                          ,  [0xD7              ], X;
]
"xlatb" = [
    []                                                                                                                                          ,  [0xD7              ], X;
]
"xor" = [
    [RegisterX64::rax, Size::Byte, OperandType::Immediate, Size::Byte]                                                                          ,  [0x34              ], X;
    [OperandType::Memory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                       ,  [0x80              ], 6, LOCK;
    [OperandType::Memory, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                         ,  [0x30              ], X, LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::Immediate, Size::Byte]                                                                      ,  [0x80              ], 6;
    [RegisterType::Legacy, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                        ,  [0x30              ], X, ENC_MR;
    [RegisterType::Legacy, Size::Byte, OperandType::LegacyMemory, Size::Byte]                                                                   ,  [0x32              ], X;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Byte]                                                                      ,  [0x83              ], 6, AUTO_SIZE  | EXACT_SIZE;
    [RegisterX64::rax, Size::Auto, OperandType::Immediate, Size::Auto]                                                                          ,  [0x35              ], X, AUTO_SIZE;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Auto]                                                                       ,  [0x81              ], 6, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, OperandType::Immediate, Size::Byte]                                                                       ,  [0x83              ], 6, AUTO_SIZE | LOCK;
    [OperandType::Memory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                         ,  [0x31              ], X, AUTO_SIZE | LOCK | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::Immediate, Size::Auto]                                                                      ,  [0x81              ], 6, AUTO_SIZE ;
    [RegisterType::Legacy, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                        ,  [0x31              ], X, AUTO_SIZE | ENC_MR;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x33              ], X, AUTO_SIZE;
]
"xorpd" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x57        ], X, PREF_66, SSE2;
]
"xorps" = [
    [RegisterType::AVX, Size::OWord, Size::Word, Size::OWord]                                                                                   ,  [0x0F, 0x57        ], X, DEFAULT, SSE;
]
"xrstor" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xAE        ], 5;
]
"xrstor64" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xAE        ], 5, WITH_REXW;
]
"xrstors64" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xC7        ], 3, WITH_REXW;
]
"xsave" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xAE        ], 4;
]
"xsave64" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xAE        ], 4, WITH_REXW;
]
"xsavec64" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xC7        ], 4, WITH_REXW;
]
"xsaveopt64" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xAE        ], 6, WITH_REXW;
]
"xsaves64" = [
    [OperandType::Memory, Size::MemAuto]                                                                                                        ,  [0x0F, 0xC7        ], 5, WITH_REXW;
]
"xsetbv" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xD1  ], X;
]
"xsha1" = [
    []                                                                                                                                          ,  [0x0F, 0xA6, 0xC8  ], X, PREF_F3, CYRIX;
]
"xsha256" = [
    []                                                                                                                                          ,  [0x0F, 0xA6, 0xD0  ], X, PREF_F3, CYRIX;
]
"xstore" = [
    []                                                                                                                                          ,  [0x0F, 0xA7, 0xC0  ], X, DEFAULT, CYRIX;
]
"xtest" = [
    []                                                                                                                                          ,  [0x0F, 0x01, 0xD6  ], X, DEFAULT, RTM;
]

"call"  = [
    [OperandType::Immediate, Size::Word, OperandType::Immediate, Size::Word]                                                                    ,  [0x9A              ], X, X86_ONLY | WORD_SIZE | EXACT_SIZE;
    [OperandType::Immediate, Size::DWord, OperandType::Immediate, Size::Word]                                                                   ,  [0x9A              ], X, X86_ONLY;
    [OperandType::Memory, Size::FWord]                                                                                                          ,  [0xFF              ], 3, X86_ONLY | EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,  [0xE8              ], X;
    [OperandType::LegacyMemory, Size::Auto]                                                                                                     ,  [0xFF              ], 2, AUTO_NO32;
]
"callf" = [
    [OperandType::Immediate, Size::Word, OperandType::Immediate, Size::Word]                                                                    ,  [0x9A              ], X, X86_ONLY | WORD_SIZE | EXACT_SIZE;
    [OperandType::Immediate, Size::DWord, OperandType::Immediate, Size::Word]                                                                   ,  [0x9A              ], X, X86_ONLY;
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xFF              ], 3, X86_ONLY | WORD_SIZE | EXACT_SIZE;
    [OperandType::Memory, Size::FWord]                                                                                                          ,  [0xFF              ], 3, X86_ONLY;
]
"jmp"   = [
    [OperandType::Immediate, Size::Word, OperandType::Immediate, Size::Word]                                                                    ,  [0x9A              ], X, X86_ONLY | WORD_SIZE | EXACT_SIZE;
    [OperandType::Immediate, Size::DWord, OperandType::Immediate, Size::Word]                                                                   ,  [0xEA              ], X, X86_ONLY;
    [OperandType::Memory, Size::FWord]                                                                                                          ,  [0xFF              ], 5, X86_ONLY | EXACT_SIZE;
    [Size::OWord, Size::Byte]                                                                                                                   ,  [0xEB              ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,  [0xE9              ], X;
    [OperandType::LegacyMemory, Size::Auto]                                                                                                     ,  [0xFF              ], 4, AUTO_NO32 ;
]
"jmpf" = [
    [OperandType::Immediate, Size::Word, OperandType::Immediate, Size::Word]                                                                    ,  [0x9A              ], X, X86_ONLY | WORD_SIZE | EXACT_SIZE;
    [OperandType::Immediate, Size::DWord, OperandType::Immediate, Size::Word]                                                                   ,  [0xEA              ], X, X86_ONLY;
    [OperandType::Memory, Size::DWord]                                                                                                          ,  [0xFF              ], 5, X86_ONLY | WORD_SIZE | EXACT_SIZE;
    [OperandType::Memory, Size::FWord]                                                                                                          ,  [0xFF              ], 5, X86_ONLY;
]
"mov"   = [
    [OperandType::LegacyMemory, Size::Auto, RegisterType::Legacy, Size::Auto]                                                                   ,  [0x89              ], X, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Byte, RegisterType::Legacy, Size::Byte]                                                                   ,  [0x88              ], X;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x8B              ], X, AUTO_SIZE;
    [RegisterType::Legacy, Size::Byte, OperandType::LegacyMemory, Size::Byte]                                                                   ,  [0x8A              ], X;
    [RegisterType::Legacy, Size::Auto, RegisterType::Segment, Size::Word]                                                                       ,  [0x8C              ], X, AUTO_SIZE;
    [OperandType::Memory, Size::Word, RegisterType::Segment, Size::Word]                                                                        ,  [0x8C              ], X;
    [RegisterType::Segment, Size::Word, OperandType::Memory, Size::Word]                                                                        ,  [0x8C              ], X;
    [RegisterType::Segment, Size::Word, RegisterType::Legacy, Size::Word]                                                                       ,  [0x8C              ], X;
    [RegisterType::Legacy, Size::Byte, OperandType::Immediate, Size::Byte]                                                                      ,  [0xB0              ], X,             SHORT_ARG;
    [RegisterType::Legacy, Size::Word, OperandType::Immediate, Size::Word]                                                                      ,  [0xB8              ], X, WORD_SIZE | SHORT_ARG;
    [RegisterType::Legacy, Size::DWord, OperandType::Immediate, Size::DWord]                                                                    ,  [0xB8              ], X,             SHORT_ARG;
    [OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Auto]                                                                 ,  [0xC7              ], 0, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Byte, OperandType::Immediate, Size::Byte]                                                                 ,  [0xC6              ], 0;
    [RegisterType::Legacy, Size::QWord, OperandType::Immediate, Size::QWord]                                                                    ,  [0xB8              ], X, WITH_REXW | SHORT_ARG;
    [RegisterType::Control, Size::DWord, RegisterType::Legacy, Size::DWord]                                                                     ,  [0x0F, 0x22        ], X; // can only match in 32 bit mode due to "cd"
    [RegisterType::Control, Size::QWord, RegisterType::Legacy, Size::QWord]                                                                     ,  [0x0F, 0x22        ], X; // doesn't need a prefix to be encoded, as it's 64 bit natural in 64 bit mode
    [RegisterType::Legacy, Size::DWord, RegisterType::Control, Size::DWord]                                                                     ,  [0x0F, 0x20        ], X;
    [RegisterType::Legacy, Size::QWord, RegisterType::Control, Size::QWord]                                                                     ,  [0x0F, 0x20        ], X;
    [ControlRegister::CR8, Size::DWord, RegisterType::Legacy, Size::DWord]                                                                      ,  [0x0F, 0x22        ], 0, PREF_F0; // note: technically CR8 should actually be encoded, but the encoding is 0.
    [ControlRegister::CR8, Size::QWord, RegisterType::Legacy, Size::QWord]                                                                      ,  [0x0F, 0x22        ], 0, PREF_F0;
    [RegisterType::Legacy, Size::DWord, ControlRegister::CR8, Size::DWord]                                                                      ,  [0x0F, 0x22        ], 0, PREF_F0;
    [RegisterType::Legacy, Size::QWord, ControlRegister::CR8, Size::QWord]                                                                      ,  [0x0F, 0x22        ], 0, PREF_F0;
    [Size::DWord, Size::DWord, RegisterType::Legacy, Size::DWord]                                                                               ,  [0x0F, 0x23        ], X; // 32 bit mode only
    [Size::DWord, Size::QWord, RegisterType::Legacy, Size::QWord]                                                                               ,  [0x0F, 0x23        ], X;
    [RegisterType::Legacy, Size::DWord, Size::DWord, Size::DWord]                                                                               ,  [0x0F, 0x21        ], X;
    [RegisterType::Legacy, Size::QWord, Size::DWord, Size::QWord]                                                                               ,  [0x0F, 0x21        ], X;
]
"movabs"  = [
    [RegisterX64::rax, Size::Byte, OperandType::Immediate, Size::QWord]                                                                         ,  [0xA0              ], X; // special syntax for 64-bit disp only mov
    [RegisterX64::rax, Size::Word, OperandType::Immediate, Size::QWord]                                                                         ,  [0xA1              ], X, WORD_SIZE;
    [RegisterX64::rax, Size::DWord, OperandType::Immediate, Size::QWord]                                                                        ,  [0xA1              ], X;
    [RegisterX64::rax, Size::QWord, OperandType::Immediate, Size::QWord]                                                                        ,  [0xA1              ], X, WITH_REXW;
    [OperandType::Immediate, Size::QWord, RegisterX64::rax, Size::Byte]                                                                         ,  [0xA2              ], X;
    [OperandType::Immediate, Size::QWord, RegisterX64::rax, Size::Word]                                                                         ,  [0xA3              ], X, WORD_SIZE;
    [OperandType::Immediate, Size::QWord, RegisterX64::rax, Size::DWord]                                                                        ,  [0xA3              ], X;
    [OperandType::Immediate, Size::QWord, RegisterX64::rax, Size::QWord]                                                                        ,  [0xA3              ], X, WITH_REXW;
]
"jo"     = [
    [Size::OWord, Size::Byte]                                                                                                                   ,  [0x70            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,  [0x0F, 0x80      ], X;
]
"jno"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,  [0x71            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,  [0x0F, 0x81      ], X;
]
"jb"     = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x72            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x82      ], X;
]
"jc"     = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x72            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x82      ], X;
]
"jnae"   = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x72            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x82      ], X;
]
"jnb"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x73            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x83      ], X;
]
"jnc"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x73            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x83      ], X;
]
"jae"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x73            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x83      ], X;
]
"jz"     = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x74            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x84      ], X;
]
"je"     = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x74            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x84      ], X;
]
"jnz"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x75            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x85      ], X;
]
"jne"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x75            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x85      ], X;
]
"jbe"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x76            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x86      ], X;
]
"jna"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x76            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x86      ], X;
]
"jnbe"   = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x77            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x87      ], X;
]
"ja"     = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x77            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x87      ], X;
]
"js"     = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x78            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x88      ], X;
]
"jns"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x79            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x89      ], X;
]
"jp"     = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x7A            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x8A      ], X;
]
"jpe"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x7A            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x8A      ], X;
]
"jnp"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x7B            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x8B      ], X;
]
"jpo"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x7B            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x8B      ], X;
]
"jl"     = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x7C            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x8C      ], X;
]
"jnge"   = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x7C            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x8C      ], X;
]
"jnl"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x7D            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x8D      ], X;
]
"jge"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x7D            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x8D      ], X;
]
"jle"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x7E            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x8E      ], X;
]
"jng"    = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x7E            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x8E      ], X;
]
"jnle"   = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x7F            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x8F      ], X;
]
"jg"     = [
    [Size::OWord, Size::Byte]                                                                                                                   ,        [0x7F            ], X, EXACT_SIZE;
    [Size::OWord, Size::DWord]                                                                                                                  ,        [0x0F, 0x8F      ], X;
]

"cmovo"    = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x40      ], X, AUTO_SIZE;
]
"cmovno"   = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x41      ], X, AUTO_SIZE;
]
"cmovb"    = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x42      ], X, AUTO_SIZE;
]
"cmovc"    = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x42      ], X, AUTO_SIZE;
]
"cmovnae"  = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x42      ], X, AUTO_SIZE;
]
"cmovnb"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x43      ], X, AUTO_SIZE;
]
"cmovnc"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x43      ], X, AUTO_SIZE;
]
"cmovae"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x43      ], X, AUTO_SIZE;
]
"cmovz"       = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x44      ], X, AUTO_SIZE;
]
"cmove"       = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x44      ], X, AUTO_SIZE;
]
"cmovnz"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x45      ], X, AUTO_SIZE;
]
"cmovne"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x45      ], X, AUTO_SIZE;
]
"cmovbe"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x46      ], X, AUTO_SIZE;
]
"cmovna"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x46      ], X, AUTO_SIZE;
]
"cmovnbe"     = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x47      ], X, AUTO_SIZE;
]
"cmova"       = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x47      ], X, AUTO_SIZE;
]
"cmovs"       = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x48      ], X, AUTO_SIZE;
]
"cmovns"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x49      ], X, AUTO_SIZE;
]
"cmovp"       = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x4A      ], X, AUTO_SIZE;
]
"cmovpe"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x4A      ], X, AUTO_SIZE;
]
"cmovnp"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x4B      ], X, AUTO_SIZE;
]
"cmovpo"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x4B      ], X, AUTO_SIZE;
]
"cmovl"       = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x4C      ], X, AUTO_SIZE;
]
"cmovnge"     = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x4C      ], X, AUTO_SIZE;
]
"cmovnl"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x4D      ], X, AUTO_SIZE;
]
"cmovge"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x4D      ], X, AUTO_SIZE;
]
"cmovle"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x4E      ], X, AUTO_SIZE;
]
"cmovng"      = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x4E      ], X, AUTO_SIZE;
]
"cmovnle"     = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x4F      ], X, AUTO_SIZE;
]
"cmovg"       = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,      [0x0F, 0x4F      ], X, AUTO_SIZE;
]

"seto"        = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x90      ], 0;
]
"setno"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x91      ], 0;
]
"setb"        = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x92      ], 0;
]
"setc"        = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x92      ], 0;
]
"setnae"      = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x92      ], 0;
]
"setnb"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x93      ], 0;
]
"setnc"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x93      ], 0;
]
"setae"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x93      ], 0;
]
"setz"        = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x94      ], 0;
]
"sete"        = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x94      ], 0;
]
"setnz"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x95      ], 0;
]
"setne"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x95      ], 0;
]
"setbe"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x96      ], 0;
]
"setna"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x96      ], 0;
]
"setnbe"      = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x97      ], 0;
]
"seta"        = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x97      ], 0;
]
"sets"        = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x98      ], 0;
]
"setns"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x99      ], 0;
]
"setp"        = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x9A      ], 0;
]
"setpe"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x9A      ], 0;
]
"setnp"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x9B      ], 0;
]
"setpo"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x9B      ], 0;
]
"setl"        = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x9C      ], 0;
]
"setnge"      = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x9C      ], 0;
]
"setnl"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x9D      ], 0;
]
"setge"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x9D      ], 0;
]
"setle"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x9E      ], 0;
]
"setng"       = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x9E      ], 0;
]
"setnle"      = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x9F      ], 0;
]
"setg"        = [
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,        [0x0F, 0x9F      ], 0;
]


"in"    = [
    [RegisterX64::rax, Size::Byte, OperandType::Immediate, Size::Byte]                                                                          ,  [0xE4            ], X;
    [RegisterX64::rax, Size::Word, OperandType::Immediate, Size::Byte]                                                                          ,  [0xE5            ], X, WORD_SIZE;
    [RegisterX64::rax, Size::DWord, OperandType::Immediate, Size::Byte]                                                                         ,  [0xE5            ], X;
    [RegisterX64::rax, Size::Byte, RegisterX64::rdx, Size::Word]                                                                                ,  [0xEC            ], X;
    [RegisterX64::rax, Size::Word, RegisterX64::rdx, Size::Word]                                                                                ,  [0xED            ], X, WORD_SIZE;
    [RegisterX64::rax, Size::DWord, RegisterX64::rdx, Size::Word]                                                                               ,  [0xED            ], X;
]

"out"   = [
    [OperandType::Immediate, Size::Byte, RegisterX64::rax, Size::Byte]                                                                          ,  [0xE6            ], X;
    [OperandType::Immediate, Size::Byte, RegisterX64::rax, Size::Word]                                                                          ,  [0xE7            ], X;
    [OperandType::Immediate, Size::Byte, RegisterX64::rax, Size::DWord]                                                                         ,  [0xE7            ], X;
    [RegisterX64::rdx, Size::Word, RegisterX64::rax, Size::Byte]                                                                                ,  [0xEE            ], X;
    [RegisterX64::rdx, Size::Word, RegisterX64::rax, Size::Word]                                                                                ,  [0xEF            ], X, WORD_SIZE;
    [RegisterX64::rdx, Size::Word, RegisterX64::rax, Size::DWord]                                                                               ,  [0xEF            ], X;
]

"crc32"  = [
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Byte]                                                                   ,  [0x0F, 0x38, 0xF0], X, AUTO_REXW | PREF_F2 | EXACT_SIZE; // unique size encoding scheme
    [RegisterType::Legacy, Size::DWord, OperandType::LegacyMemory, Size::Word]                                                                  ,  [0x0F, 0x38, 0xF1], X, WORD_SIZE | PREF_F2 | EXACT_SIZE;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x0F, 0x38, 0xF1], X, AUTO_REXW | PREF_F2 | EXACT_SIZE;
]

"imul"   = [
    [OperandType::LegacyMemory, Size::Auto]                                                                                                     ,  [0xF7            ], 5, AUTO_SIZE;
    [OperandType::LegacyMemory, Size::Byte]                                                                                                     ,  [0xF6            ], 5;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto]                                                                   ,  [0x0F, 0xAF      ], X, AUTO_SIZE;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Byte]                               ,  [0x6B            ], X, AUTO_SIZE | EXACT_SIZE;
    [RegisterType::Legacy, Size::Auto, OperandType::LegacyMemory, Size::Auto, OperandType::Immediate, Size::Auto]                               ,  [0x69            ], X, AUTO_SIZE;
]

)
