/*
 * ControllerLateral_private.h
 *
 * Academic License - for use in teaching, academic research, and meeting
 * course requirements at degree granting institutions only.  Not for
 * government, commercial, or other organizational use.
 *
 * Code generation for model "ControllerLateral".
 *
 * Model version              : 6.194
 * Simulink Coder version : 9.5 (R2021a) 14-Nov-2020
 * C source code generated on : Mon May 20 14:18:02 2024
 *
 * Target selection: pmp.tlc
 * Note: GRT includes extra infrastructure and instrumentation for prototyping
 * Embedded hardware selection: ARM Compatible->ARM 64-bit (LP64)
 * Code generation objectives: Unspecified
 * Validation result: Not run
 */

#ifndef RTW_HEADER_ControllerLateral_private_h_
#define RTW_HEADER_ControllerLateral_private_h_
#include "rtwtypes.h"
#include "multiword_types.h"
#include "zero_crossing_types.h"
#include "ControllerLateral.h"
#if !defined(rt_VALIDATE_MEMORY)
#define rt_VALIDATE_MEMORY(S, ptr)     if(!(ptr)) {\
 rtmSetErrorStatus(ControllerLateral_M, RT_MEMORY_ALLOCATION_ERROR);\
 }
#endif

#if !defined(rt_FREE)
#if !defined(_WIN32)
#define rt_FREE(ptr)                   if((ptr) != (NULL)) {\
 free((ptr));\
 (ptr) = (NULL);\
 }
#else

/* Visual and other windows compilers declare free without const */
#define rt_FREE(ptr)                   if((ptr) != (NULL)) {\
 free((void *)(ptr));\
 (ptr) = (NULL);\
 }
#endif
#endif

extern real_T look2_pbinlxpw(real_T u0, real_T u1, const real_T bp0[], const
  real_T bp1[], const real_T table[], uint32_T prevIndex[], const uint32_T
  maxIndex[], uint32_T stride);
extern real_T look1_pbinlxpw(real_T u0, const real_T bp0[], const real_T table[],
  uint32_T prevIndex[], uint32_T maxIndex);
extern uint32_T plook_binxp(real_T u, const real_T bp[], uint32_T maxIndex,
  real_T *fraction, uint32_T *prevIndex);
extern real_T intrp2d_l_pw(const uint32_T bpIndex[], const real_T frac[], const
  real_T table[], const uint32_T stride);
extern uint32_T binsearch_u32d_prevIdx(real_T u, const real_T bp[], uint32_T
  startIndex, uint32_T maxIndex);
void ControllerLateral_MovingAverage_Init(DW_MovingAverage_ControllerLateral_T
  *localDW);
void ControllerLateral_MovingAverage_Start(DW_MovingAverage_ControllerLateral_T *
  localDW);
void ControllerLateral_MovingAverage(real_T rtu_0,
  B_MovingAverage_ControllerLateral_T *localB,
  DW_MovingAverage_ControllerLateral_T *localDW);
void ControllerLateral_MovingAverage_p_Init
  (DW_MovingAverage_ControllerLateral_f_T *localDW);
void ControllerLateral_MovingAverage_n_Start
  (DW_MovingAverage_ControllerLateral_f_T *localDW);
void ControllerLateral_MovingAverage_p(real_T rtu_0, real_T rtu_1,
  B_MovingAverage_ControllerLateral_c_T *localB,
  DW_MovingAverage_ControllerLateral_f_T *localDW);
void ControllerLateral_MATLABFunction(real_T rtu_mode,
  B_MATLABFunction_ControllerLateral_T *localB);
void ControllerLateral_MovingAverage_Term(DW_MovingAverage_ControllerLateral_T
  *localDW);
void ControllerLateral_MovingAverage_d_Term
  (DW_MovingAverage_ControllerLateral_f_T *localDW);
void ControllerLateral_MATLABFunction_Term(void);

#endif                             /* RTW_HEADER_ControllerLateral_private_h_ */
