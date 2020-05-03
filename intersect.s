<lastone::geometry::model::Model as lastone::geometry::Intersectable>::intersect::{{closure}} (src\geometry\model.rs:30):
 push    rbp
 push    r15
 push    r14
 push    rsi
 push    rdi
 push    rbx
 sub     rsp, 1032
 lea     rbp, [rsp, +, 128]
 vmovaps xmmword, ptr, [rbp, +, 880], xmm15
 vmovaps xmmword, ptr, [rbp, +, 864], xmm14
 vmovaps xmmword, ptr, [rbp, +, 848], xmm13
 vmovaps xmmword, ptr, [rbp, +, 832], xmm12
 vmovaps xmmword, ptr, [rbp, +, 816], xmm11
 vmovaps xmmword, ptr, [rbp, +, 800], xmm10
 vmovaps xmmword, ptr, [rbp, +, 784], xmm9
 vmovaps xmmword, ptr, [rbp, +, 768], xmm8
 vmovaps xmmword, ptr, [rbp, +, 752], xmm7
 vmovaps xmmword, ptr, [rbp, +, 736], xmm6
 and     rsp, -32
 mov     r10, qword, ptr, [r8, +, 16]
 mov     r9, rdx
 movabs  rdx, -6148914691236517205
 mov     rax, r10
 mov     r11, r10
 mul     rdx
 shr     rdx
 lea     rax, [rdx, +, 2*rdx]
 sub     r11, rax
 mov     rax, r10
 sub     rax, r11
 jb      .LBB95_36
 mov     rdx, qword, ptr, [r9]
 mov     rbx, qword, ptr, [r9, +, 8]
 mov     rdi, qword, ptr, [r9, +, 24]
 vbroadcastss ymm15, dword, ptr, [rip, +, __real@7f800000]
 mov     r10, qword, ptr, [rdx]
 mov     rdx, qword, ptr, [rbx]
 mov     rbx, qword, ptr, [r9, +, 16]
 vmovaps ymmword, ptr, [rsp, +, 384], ymm15
 vmovaps ymmword, ptr, [rsp, +, 160], ymm15
 vmovss  xmm0, dword, ptr, [rdx, +, 12]
 vmovaps xmmword, ptr, [rsp, +, 80], xmm0
 vmovss  xmm0, dword, ptr, [rdx, +, 16]
 vmovaps xmmword, ptr, [rsp, +, 64], xmm0
 vmovss  xmm0, dword, ptr, [rdx, +, 20]
 vmovaps xmmword, ptr, [rsp, +, 48], xmm0
 vbroadcastss ymm0, dword, ptr, [rbx]
 vmovaps ymmword, ptr, [rsp, +, 416], ymm0
 vbroadcastss ymm0, dword, ptr, [rdi]
 vmovaps ymmword, ptr, [rsp, +, 352], ymm0
 vxorps  xmm0, xmm0, xmm0
 vmovaps ymmword, ptr, [rsp, +, 128], ymm0
 vxorps  xmm0, xmm0, xmm0
 vmovaps ymmword, ptr, [rsp, +, 96], ymm0
 vxorps  xmm0, xmm0, xmm0
 vmovaps ymmword, ptr, [rsp, +, 224], ymm0
 vxorps  xmm0, xmm0, xmm0
 vmovaps ymmword, ptr, [rsp, +, 192], ymm0
 cmp     rax, 3
 jb      .LBB95_4
 vbroadcastss ymm0, dword, ptr, [rdx]
 vbroadcastss ymm1, dword, ptr, [rip, +, __real@bf800000]
 vmovaps ymmword, ptr, [rsp, +, 672], ymm0
 vbroadcastss ymm0, dword, ptr, [rdx, +, 4]
 vmovaps ymmword, ptr, [rsp, +, 256], ymm1
 vbroadcastss ymm1, dword, ptr, [rip, +, __real@3f800000]
 vmovaps ymmword, ptr, [rsp, +, 320], ymm0
 vbroadcastss ymm0, dword, ptr, [rdx, +, 8]
 vmovaps ymmword, ptr, [rsp, +, 512], ymm1
 vbroadcastss ymm1, dword, ptr, [rip, +, __real@7fffffff]
 mov     rdx, qword, ptr, [r8]
 vmovaps ymmword, ptr, [rsp, +, 288], ymm0
 vbroadcastss ymm0, dword, ptr, [rsp, +, 80]
 vmovaps ymmword, ptr, [rsp, +, 480], ymm1
 vbroadcastss ymm1, dword, ptr, [rip, +, __real@33d6bf95]
 vmovaps ymmword, ptr, [rsp, +, 640], ymm0
 vbroadcastss ymm0, dword, ptr, [rsp, +, 64]
 vmovaps ymmword, ptr, [rsp, +, 448], ymm1
 vmovaps ymm9, ymmword, ptr, [rsp, +, 640]
 vmovaps ymm7, ymmword, ptr, [rsp, +, 448]
 vmovaps ymmword, ptr, [rsp, +, 608], ymm0
 vbroadcastss ymm0, dword, ptr, [rsp, +, 48]
 vmovaps ymmword, ptr, [rsp, +, 576], ymm0
 vbroadcastss ymm0, dword, ptr, [rip, +, __real@7f800000]
 vmovaps ymm15, ymmword, ptr, [rsp, +, 576]
 vmovaps ymmword, ptr, [rsp, +, 544], ymm0
 vmovaps ymmword, ptr, [rsp, +, 160], ymm0
 vxorps  xmm0, xmm0, xmm0
 vmovaps ymmword, ptr, [rsp, +, 192], ymm0
 vxorps  xmm0, xmm0, xmm0
 vmovaps ymmword, ptr, [rsp, +, 224], ymm0
 vxorps  xmm0, xmm0, xmm0
 vmovaps ymmword, ptr, [rsp, +, 96], ymm0
 vxorps  xmm0, xmm0, xmm0
 vmovaps ymmword, ptr, [rsp, +, 128], ymm0
.LBB95_3:
 vmovaps ymm3, ymmword, ptr, [rdx]
 vmovaps ymm8, ymmword, ptr, [rdx, +, 192]
 vmovaps ymm0, ymmword, ptr, [rdx, +, 96]
 vmovaps ymm1, ymmword, ptr, [rdx, +, 32]
 vmovaps ymm5, ymmword, ptr, [rdx, +, 64]
 vmovaps ymm10, ymmword, ptr, [rdx, +, 224]
 vmovaps ymm6, ymmword, ptr, [rdx, +, 256]
 vmovaps ymm4, ymmword, ptr, [rdx, +, 128]
 vmovaps ymm13, ymmword, ptr, [rdx, +, 160]
 add     rax, -3
 add     rdx, 288
 vsubps  ymm2, ymm8, ymm3
 vmovaps ymm8, ymmword, ptr, [rsp, +, 608]
 vsubps  ymm12, ymm0, ymm3
 vsubps  ymm0, ymm10, ymm1
 vsubps  ymm10, ymm6, ymm5
 vsubps  ymm11, ymm4, ymm1
 vsubps  ymm4, ymm13, ymm5
 vmulps  ymm6, ymm9, ymm10
 vmovaps ymmword, ptr, [rsp, +, 704], ymm2
 vfmsub231ps ymm6, ymm15, ymm2
 vmulps  ymm13, ymm8, ymm2
 vmovaps ymm2, ymmword, ptr, [rsp, +, 320]
 vfmsub231ps ymm13, ymm9, ymm0
 vsubps  ymm2, ymm2, ymm1
 vmovaps ymm1, ymmword, ptr, [rsp, +, 288]
 vmulps  ymm14, ymm4, ymm13
 vfmadd231ps ymm14, ymm11, ymm6
 vsubps  ymm1, ymm1, ymm5
 vmulps  ymm5, ymm1, ymm13
 vmovaps ymm13, ymmword, ptr, [rsp, +, 672]
 vfmadd231ps ymm5, ymm2, ymm6
 vmulps  ymm6, ymm15, ymm0
 vfmsub231ps ymm6, ymm8, ymm10
 vsubps  ymm3, ymm13, ymm3
 vfmadd231ps ymm14, ymm12, ymm6
 vfmadd231ps ymm5, ymm3, ymm6
 vmulps  ymm6, ymm1, ymm11
 vfmsub231ps ymm6, ymm2, ymm4
 vmulps  ymm4, ymm3, ymm4
 vmulps  ymm2, ymm2, ymm12
 vfmsub213ps ymm1, ymm12, ymm4
 vmovaps ymm12, ymmword, ptr, [rsp, +, 512]
 vfmsub231ps ymm2, ymm3, ymm11
 vandps  ymm3, ymm14, ymmword, ptr, [rsp, +, 480]
 vmulps  ymm10, ymm2, ymm10
 vmulps  ymm2, ymm15, ymm2
 vdivps  ymm4, ymm12, ymm14
 vxorps  xmm14, xmm14, xmm14
 vcmpltps ymm3, ymm7, ymm3
 vfmadd231ps ymm2, ymm8, ymm1
 vfmadd231ps ymm10, ymm0, ymm1
 vfmadd231ps ymm10, ymm6, ymmword, ptr, [rsp, +, 704]
 vfmadd231ps ymm2, ymm9, ymm6
 vmulps  ymm5, ymm5, ymm4
 vcmpleps ymm0, ymm5, ymm12
 vcmpleps ymm11, ymm14, ymm5
 vextractf128 xmm1, ymm0, 1
 vandps  ymm3, ymm11, ymm3
 vpackssdw xmm0, xmm0, xmm1
 vmulps  ymm1, ymm2, ymm4
 vmulps  ymm4, ymm10, ymm4
 vcmpleps ymm2, ymm14, ymm1
 vaddps  ymm1, ymm5, ymm1
 vextractf128 xmm5, ymm2, 1
 vcmpleps ymm1, ymm1, ymm12
 vpackssdw xmm2, xmm2, xmm5
 vcmpltps ymm5, ymm7, ymm4
 vpand   xmm0, xmm0, xmm2
 vextractf128 xmm2, ymm1, 1
 vextractf128 xmm6, ymm5, 1
 vpackssdw xmm1, xmm1, xmm2
 vextractf128 xmm2, ymm3, 1
 vpackssdw xmm5, xmm5, xmm6
 vpackssdw xmm2, xmm3, xmm2
 vpand   xmm0, xmm0, xmm5
 vmovaps ymm3, ymmword, ptr, [rsp, +, 160]
 vpand   xmm0, xmm2, xmm0
 vpand   xmm0, xmm0, xmm1
 vmovaps ymm1, ymmword, ptr, [rsp, +, 544]
 vpmovzxwd ymm0, xmm0
 vpslld  ymm0, ymm0, 31
 vblendvps ymm0, ymm1, ymm4, ymm0
 vmovaps ymm4, ymmword, ptr, [rsp, +, 96]
 vmulps  ymm1, ymm9, ymm0
 vcmpltps ymm2, ymm3, ymm0
 vminps  ymm3, ymm3, ymm0
 vmovaps ymmword, ptr, [rsp, +, 160], ymm3
 vaddps  ymm1, ymm13, ymm1
 vblendvps ymm4, ymm1, ymm4, ymm2
 vmulps  ymm1, ymm8, ymm0
 vaddps  ymm1, ymm1, ymmword, ptr, [rsp, +, 320]
 vmulps  ymm0, ymm15, ymm0
 vaddps  ymm0, ymm0, ymmword, ptr, [rsp, +, 288]
 vmovaps ymmword, ptr, [rsp, +, 96], ymm4
 vmovaps ymm4, ymmword, ptr, [rsp, +, 128]
 vblendvps ymm4, ymm1, ymm4, ymm2
 vmovaps ymm1, ymmword, ptr, [rsp, +, 224]
 vmovaps ymmword, ptr, [rsp, +, 128], ymm4
 vblendvps ymm1, ymm0, ymm1, ymm2
 vmovaps ymm0, ymmword, ptr, [rsp, +, 192]
 vmovaps ymmword, ptr, [rsp, +, 224], ymm1
 vmovaps ymm1, ymmword, ptr, [rsp, +, 256]
 vaddps  ymm1, ymm1, ymm12
 vblendvps ymm0, ymm1, ymm0, ymm2
 vmovaps ymmword, ptr, [rsp, +, 256], ymm1
 vmovaps ymmword, ptr, [rsp, +, 192], ymm0
 cmp     rax, 2
 ja      .LBB95_3
.LBB95_4:
 vmovaps ymm3, ymmword, ptr, [rsp, +, 160]
 vmovaps ymm1, ymmword, ptr, [rsp, +, 416]
 vcmpltps ymm0, ymm3, ymmword, ptr, [rsp, +, 352]
 vmovaps ymm4, ymmword, ptr, [rsp, +, 384]
 vmovaps ymm5, ymmword, ptr, [rsp, +, 96]
 vmovaps ymm2, ymmword, ptr, [rsp, +, 128]
 mov     dl, 2
 vcmpltps ymm1, ymm1, ymm3
 vandps  ymm0, ymm1, ymm0
 vblendvps ymm0, ymm4, ymm3, ymm0
 vmovaps ymm3, ymmword, ptr, [rsp, +, 192]
 vmovaps ymm4, ymmword, ptr, [rsp, +, 224]
 vmovshdup xmm7, xmm0
 vucomiss xmm0, xmm7
 setae   al
 sub     dl, al
 dec     al
 vucomiss xmm7, xmm0
 movzx   eax, al
 movzx   r9d, dl
 cmovae  r9d, eax
 cmp     r9b, 2
 je      .LBB95_37
 cmp     r9b, 1
 je      .LBB95_7
 vmovaps xmm7, xmm0
.LBB95_7:
 vpermilpd xmm1, xmm0, 1
 mov     dl, 2
 vucomiss xmm7, xmm1
 setae   al
 sub     dl, al
 dec     al
 vucomiss xmm1, xmm7
 movzx   eax, al
 movzx   r11d, dl
 cmovae  r11d, eax
 cmp     r11b, 2
 je      .LBB95_37
 cmp     r11b, 1
 jne     .LBB95_10
 vmovapd xmm7, xmm1
.LBB95_10:
 vpermilps xmm1, xmm0, 231
 mov     dl, 2
 vucomiss xmm7, xmm1
 setae   al
 sub     dl, al
 dec     al
 vucomiss xmm1, xmm7
 movzx   eax, al
 movzx   r14d, dl
 cmovae  r14d, eax
 cmp     r14b, 2
 je      .LBB95_37
 cmp     r14b, 1
 jne     .LBB95_13
 vmovapd xmm7, xmm1
.LBB95_13:
 vextractf128 xmm0, ymm0, 1
 mov     dl, 2
 vucomiss xmm7, xmm0
 setae   al
 sub     dl, al
 dec     al
 vucomiss xmm0, xmm7
 movzx   eax, al
 movzx   r15d, dl
 cmovae  r15d, eax
 cmp     r15b, 2
 je      .LBB95_37
 cmp     r15b, 1
 jne     .LBB95_16
 vmovapd xmm7, xmm0
.LBB95_16:
 vmovshdup xmm1, xmm0
 mov     dl, 2
 vucomiss xmm7, xmm1
 setae   al
 sub     dl, al
 dec     al
 vucomiss xmm1, xmm7
 movzx   eax, al
 movzx   esi, dl
 cmovae  esi, eax
 cmp     sil, 2
 je      .LBB95_37
 cmp     sil, 1
 jne     .LBB95_19
 vmovaps xmm7, xmm1
.LBB95_19:
 vpermilpd xmm1, xmm0, 1
 mov     dl, 2
 vucomiss xmm7, xmm1
 setae   al
 sub     dl, al
 dec     al
 vucomiss xmm1, xmm7
 movzx   eax, al
 movzx   edi, dl
 cmovae  edi, eax
 cmp     dil, 2
 je      .LBB95_37
 cmp     dil, 1
 jne     .LBB95_22
 vmovapd xmm7, xmm1
.LBB95_22:
 vpermilps xmm0, xmm0, 231
 mov     dl, 2
 vucomiss xmm7, xmm0
 setae   al
 sub     dl, al
 dec     al
 vucomiss xmm0, xmm7
 movzx   eax, al
 movzx   ebx, dl
 cmovae  ebx, eax
 cmp     bl, 2
 je      .LBB95_37
 cmp     bl, 1
 jne     .LBB95_25
 vmovapd xmm7, xmm0
.LBB95_25:
 vucomiss xmm7, dword, ptr, [rip, +, __real@7f800000]
 jb      .LBB95_26
 mov     byte, ptr, [rcx, +, 36], 2
 jmp     .LBB95_35
.LBB95_26:
 xor     eax, eax
 cmp     r9b, 1
 mov     edx, 2
 mov     r9d, 6
 vmovaps ymmword, ptr, [rsp, +, 832], ymm5
 sete    al
 cmp     r11b, 1
 mov     r11d, 4
 cmovne  edx, eax
 cmp     r14b, 1
 mov     eax, 3
 cmovne  eax, edx
 cmp     r15b, 1
 movabs  rdx, -9223372036854775808
 cmovne  r11d, eax
 cmp     sil, 1
 mov     eax, 5
 cmovne  eax, r11d
 cmp     dil, 1
 cmovne  r9d, eax
 cmp     bl, 1
 mov     ebx, 7
 cmovne  ebx, r9d
 mov     eax, ebx
 and     eax, 7
 vmovss  xmm8, dword, ptr, [rsp, +, 4*rax, +, 832]
 vmovaps ymmword, ptr, [rsp, +, 800], ymm2
 vmovss  xmm9, dword, ptr, [rsp, +, 4*rax, +, 800]
 vmovaps ymmword, ptr, [rsp, +, 768], ymm4
 vmovss  xmm4, dword, ptr, [rip, +, __real@5f000000]
 vmovss  xmm2, dword, ptr, [rsp, +, 4*rax, +, 768]
 vmovaps ymmword, ptr, [rsp, +, 736], ymm3
 vmovss  xmm3, dword, ptr, [rsp, +, 4*rax, +, 736]
 vsubss  xmm5, xmm3, xmm4
 vcvttss2si rax, xmm5
 xor     rdx, rax
 vcvttss2si rax, xmm3
 vucomiss xmm4, xmm3
 cmovbe  rax, rdx
 mov     rdx, qword, ptr, [r8, +, 40]
 cmp     rdx, rax
 jbe     .LBB95_27
 lea     rax, [rax, +, 2*rax]
 movsxd  rdx, ebx
 vmovaps xmm1, xmmword, ptr, [rsp, +, 64]
 vmovaps xmm0, xmmword, ptr, [rsp, +, 80]
 shl     rax, 5
 add     rax, qword, ptr, [r8, +, 24]
 vmovss  xmm5, dword, ptr, [rax, +, 4*rdx, +, 64]
 vmulss  xmm6, xmm5, dword, ptr, [rsp, +, 48]
 vmovss  xmm4, dword, ptr, [rax, +, 4*rdx, +, 32]
 vmovss  xmm3, dword, ptr, [rax, +, 4*rdx]
 vfmadd213ss xmm1, xmm4, xmm6
 vxorps  xmm6, xmm6, xmm6
 vfmadd213ss xmm0, xmm3, xmm1
 vucomiss xmm6, xmm0
 vmovaps xmm1, xmm0
 ja      .LBB95_31
 vbroadcastss xmm6, dword, ptr, [rip, +, __real@80000000]
 vxorps  xmm3, xmm3, xmm6
 vxorps  xmm4, xmm4, xmm6
 vxorps  xmm5, xmm5, xmm6
.LBB95_31:
 mov     rax, qword, ptr, [r8, +, 48]
 mov     rdx, qword, ptr, [r10, +, 40]
 cmp     rdx, rax
 jbe     .LBB95_32
 mov     dl, byte, ptr, [rsp, +, 47]
 lea     rax, [rax, +, 2*rax]
 movzx   r8d, word, ptr, [rsp, +, 45]
 vxorps  xmm6, xmm6, xmm6
 shl     rax, 3
 add     rax, qword, ptr, [r10, +, 24]
 vucomiss xmm6, xmm1
 mov     qword, ptr, [rcx], rax
 vmovss  dword, ptr, [rcx, +, 8], xmm8
 vmovss  dword, ptr, [rcx, +, 12], xmm9
 vmovss  dword, ptr, [rcx, +, 16], xmm2
 vmovss  dword, ptr, [rcx, +, 20], xmm3
 vmovss  dword, ptr, [rcx, +, 24], xmm4
 vmovss  dword, ptr, [rcx, +, 28], xmm5
 vmovss  dword, ptr, [rcx, +, 32], xmm7
 seta    byte, ptr, [rcx, +, 36]
 mov     word, ptr, [rcx, +, 37], r8w
 mov     byte, ptr, [rcx, +, 39], dl
.LBB95_35:
 vmovaps xmm6, xmmword, ptr, [rbp, +, 736]
 vmovaps xmm7, xmmword, ptr, [rbp, +, 752]
 vmovaps xmm8, xmmword, ptr, [rbp, +, 768]
 vmovaps xmm9, xmmword, ptr, [rbp, +, 784]
 vmovaps xmm10, xmmword, ptr, [rbp, +, 800]
 vmovaps xmm11, xmmword, ptr, [rbp, +, 816]
 vmovaps xmm12, xmmword, ptr, [rbp, +, 832]
 vmovaps xmm13, xmmword, ptr, [rbp, +, 848]
 vmovaps xmm14, xmmword, ptr, [rbp, +, 864]
 vmovaps xmm15, xmmword, ptr, [rbp, +, 880]
 lea     rsp, [rbp, +, 904]
 pop     rbx
 pop     rdi
 pop     rsi
 pop     r14
 pop     r15
 pop     rbp
 vzeroupper
 ret
.LBB95_37:
 lea     rcx, [rip, +, __unnamed_2]
 lea     r8, [rip, +, __unnamed_69]
 mov     edx, 43
 vzeroupper
 call    core::panicking::panic
 ud2
.LBB95_36:
 lea     r8, [rip, +, __unnamed_70]
 mov     rcx, rax
 mov     rdx, r10
 call    core::slice::slice_index_len_fail
 ud2
.LBB95_27:
 lea     r8, [rip, +, __unnamed_71]
 mov     rcx, rax
 vzeroupper
 call    core::panicking::panic_bounds_check
 ud2
.LBB95_32:
 lea     r8, [rip, +, __unnamed_72]
 mov     rcx, rax
 vzeroupper
 call    core::panicking::panic_bounds_check
 ud2
