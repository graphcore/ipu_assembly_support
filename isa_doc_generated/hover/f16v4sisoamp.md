* `f16v4sisoamp $aDst0:Dst0+1, $aSrc0:Src0+1, $aSrc1:Src1+1, enumFlags`

*f16* floating-point accumulating matrix-vector product. Input and
result partial-sums are 2 x *single-precision* values.

::: tabularcolumns
p{dimexpr 0.07linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}\>{centering}p{dimexpr
0.11linewidth-2tabcolsep}p{dimexpr 0.05linewidth-2tabcolsep}

::: rst-class
fullwidth

  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  \$aSrc0\|1     \$AACC\[14\]            \$AACC\[12\]            \$AACC\[10\]            \$AACC\[8\]             \$AACC\[6\]             \$AACC\[4\]             \$AACC\[2\]             \$AACC\[0\]             \$aDst0
  -------------- ----------------------- ----------------------- ----------------------- ----------------------- ----------------------- ----------------------- ----------------------- ----------------------- ---------
  0[^1]\|P0,P1   **-**                   **-**                   **-**                   **-**                   **-**                   **-**                   **-**                   **-**                   **-**

  0\|P2,P3       **-**                   **-**                   **-**                   \[**WARM-UP**           **PERIOD**\]            **-**                   **-**                   **-**                   **-**

  0\|P4,P5       **-**                   **-**                   **-**                   **-**                   **-**                   **-**                   **-**                   **-**                   **-**

  0\| P6,P7      **-**                   **-**                   **-**                   **-**                   **-**                   **-**                   **-**                   **-**                   **-**

  x0\|P8,P9      R7=x0.**CW**7,0+P7      R6=x0.**CW**6,0+P6      R5=x0.**CW**5,0+P5      R4=x0.**CW**4,0+P4      R3=x0.**CW**3,0+P3      R2=x0.**CW**2,0+P2      R1=x0.**CW**1,0+P1      R0=x0.**CW**0,0+P0      **-**

  x1\|P10,P11    R7+=x1.**CW**7,1        R6+=x1.**CW**6,1        R5+=x1.**CW**5,1        R4+=x1.**CW**4,1        R3+=x1.**CW**3,1        R2+=x1.**CW**2,1        R1+=x1.**CW**1,1        R0+=x1.**CW**0,1        **-**

  x2\|P12,P13    R7+=x2.**CW**7,2        R6+=x2.**CW**6,2        R5+=x2.**CW**5,2        R4+=x2.**CW**4,2        R3+=x2.**CW**3,2        R2+=x2.**CW**2,2        R1+=x2.**CW**1,2        R0+=x2.**CW**0,2        **-**

  x3\|P14,P15    R7+=x3.**CW**7,3        R6+=x3.**CW**6,3        R5+=x3.**CW**5,3        R4+=x3.**CW**4,3        R3+=x3.**CW**3,3        R2+=x3.**CW**2,3        R1+=x3.**CW**1,3        R0+=x3.**CW**0,3        **-**

  x4\|P16,P17    R15=x4.**CW**7,0+P15    R14=x4.**CW**6,0+P14    R13=x4.**CW**5,0+P13    R12=x4.**CW**4,0+P12    R11=x4.**CW**3,0+P11    R10=x4.**CW**2,0+P10    R9=x4.**CW**1,0+P9      R8=x4.**CW**0,0+P8      R0,R1

  x5\|P18,P19    R15+=x5.**CW**7,1       R14+=x5.**CW**6,1       R13+=x5.**CW**5,1       R12+=x5.**CW**4,1       R11+=x5.**CW**3,1       R10+=x5.**CW**2,1       R9+=x5.**CW**1,1        R8+=x5.**CW**0,1        R2,R3

  x6\|P20,P21    R15+=x6.**CW**7,2       R14+=x6.**CW**6,2       R13+=x6.**CW**5,2       R12+=x6.**CW**4,2       R11+=x6.**CW**3,2       R10+=x6.**CW**2,2       R9+=x6.**CW**1,2        R8+=x6.**CW**0,2        R4,R5

  x7\|P22,P23    R15+=x7.**CW**7,3       R14+=x7.**CW**6,3       R13+=x7.**CW**5,3       R12+=x7.**CW**4,3       R11+=x7.**CW**3,3       R10+=x7.**CW**2,3       R9+=x7.**CW**1,3        R8+=x7.**CW**0,3        R6,R7

  x8\|P24,P25    R23=x8.**CW**7,0+P23    R22=x8.**CW**6,0+P22    R21=x8.**CW**5,0+P21    R20=x8.**CW**4,0+P20    R19=x8.**CW**3,0+P19    R18=x8.**CW**2,0+P18    R17=x8.**CW**1,0+P17    R16=x8.**CW**0,0+P16    R8,R9

  x9\|P26,P27    R23+=x9.**CW**7,1       R22+=x9.**CW**6,1       R21+=x9.**CW**5,1       R20+=x9.**CW**4,1       R19+=x9.**CW**3,1       R18+=x9.**CW**2,1       R17+=x9.**CW**1,1       R16+=x9.**CW**0,1       R10,R11

  x10\|P28,P29   R23+=x10.**CW**7,2      R22+=x10.**CW**6,2      R21+=x10.**CW**5,2      R20+=x10.**CW**4,2      R19+=x10.**CW**3,2      R18+=x10.**CW**2,2      R17+=x10.**CW**1,2      R16+=x10.**CW**0,2      R12,R13

  x11\|P30,P31   R23+=x11.**CW**7,3      R22+=x11.**CW**6,3      R21+=x11.**CW**5,3      R20+=x11.**CW**4,3      R19+=x11.**CW**3,3      R18+=x11.**CW**2,3      R17+=x11.**CW**1,3      R16+=x11.**CW**0,3      R14,R15

  x12\|P32,P33   R31=x12.**CW**7,0+P31   R30=x12.**CW**6,0+P30   R29=x12.**CW**5,0+P29   R28=x12.**CW**4,0+P28   R27=x12.**CW**3,0+P27   R26=x12.**CW**2,0+P26   R25=x12.**CW**1,0+P25   R24=x12.**CW**0,0+P24   R16,R17
  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

  : f16v4sisoamp 8x1x1x16 example sequence

Pn is *single-precision* input partial-sum *n* xn is an *f16v4* input
vector **CW**m,n is the common weight state **\$CWEI**\_*m*\_*n* Rn is
the final *single-precision* result of successive dot-product
accumulations that began with Pn

enumFlags format:

![f16v4sisoamp immediate
format](images/autogen/F16AMP_ENUMFLAGS.*){.align-center}

8 output channels are processed/produced.

[^1]: 0 input used to fill AMP pipeline during warm-up period
